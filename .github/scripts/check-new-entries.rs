#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
anyhow = "1.0.104"
chrono = { version = "0.4.45", default-features = false }
indoc = "2.0.8"
itertools = "0.15.0"
octocrab = { version = "0.54.9", default-features = false, features = ["default-client", "rustls", "rustls-ring"] }
tokio = { version = "1.54.1", features = ["macros", "rt"] }
---

//! Checks that each GitHub project added to README.md by a pull request is
//! established enough to list. Run by .github/workflows/new-entries.yml.
//!
//! A project passes when, as of the day the pull request was opened:
//! - it is at least `MIN_AGE_DAYS` old, counted from the repository's creation or
//!   its first commit, whichever is later (the same rule as sindresorhus/awesome);
//! - it has commits on at least `MIN_ACTIVE_DAYS` different days, so a repo
//!   created and pushed in one sitting does not count as established.
//!
//! A project with at least `MIN_STARS` stars passes either way. Stars are counted
//! now, since GitHub doesn't keep a history of them.
//!
//! Pass the pull request number as the only argument. To run it locally:
//!
//! ```sh
//! GITHUB_TOKEN=$(gh auth token) .github/scripts/check-new-entries.rs 591
//! ```

use std::collections::HashSet;
use std::{env, fs};

use anyhow::{Context, Result, bail};
use chrono::{DateTime, Utc};
use indoc::formatdoc;
use itertools::{Either, Itertools};
use octocrab::Octocrab;
use octocrab::models::repos::RepoCommit;
use octocrab::pulls::PullRequestHandler;
use octocrab::repos::RepoHandler;

const MIN_AGE_DAYS: i64 = 30;
const MIN_ACTIVE_DAYS: i64 = 5;
const MIN_STARS: u32 = 50;

/// Checks the entries a pull request adds to README.md, writes a table of the
/// results to the job summary, and exits with an error if any project is too
/// new. Entries that don't link to GitHub can't be checked, so they get a row
/// asking for a manual check, and a warning.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN is not set")?;
    let github = Octocrab::builder().personal_token(token).build()?;
    let number = env::args().nth(1).context("pass a pull request number")?;

    let pulls = github.pulls("ratatui", "awesome-ratatui");
    let pr = pulls.get(number.parse()?).await?;
    let opened_at = pr.created_at.context("pull request has no creation date")?;
    let patch = readme_patch(&pulls, pr.number).await?;

    let (repos, others) = new_entries(&patch);
    if repos.is_empty() && others.is_empty() {
        println!("No new entries in README.md.");
        return Ok(());
    }

    let mut results = Vec::new();
    for (owner, repo) in &repos {
        results.push(check_repo(&github, owner, repo, opened_at).await?);
    }

    write_summary(&results, &others)?;
    for url in &others {
        println!("::warning::{url} is not a GitHub link, so check its age by hand.");
    }
    fail_if_any_problems(&results)
}

/// Fetches the pull request's diff of README.md. The workflow only runs when
/// README.md changes, so a missing file is an error. GitHub leaves the patch out
/// for very large diffs, which is also an error rather than a silent pass.
async fn readme_patch(pulls: &PullRequestHandler<'_>, number: u64) -> Result<String> {
    let files = pulls.list_files(number).await?;
    let readme = files
        .items
        .into_iter()
        .find(|file| file.filename == "README.md");
    let readme = readme.context("pull request doesn't change README.md")?;
    readme
        .patch
        .context("README.md diff is too large for GitHub to include")
}

/// Finds the links of the entries the patch adds and splits them into GitHub
/// repositories (owner and repo) and other links. An entry whose link is both
/// removed and added was moved or edited rather than added, so it is left out.
fn new_entries(patch: &str) -> (Vec<(&str, &str)>, Vec<&str>) {
    let mut added = Vec::new();
    let mut removed = HashSet::new();
    for line in patch.lines() {
        if let Some(url) = line.strip_prefix('+').and_then(entry_url) {
            added.push(url);
        } else if let Some(url) = line.strip_prefix('-').and_then(entry_url) {
            removed.insert(url);
        }
    }

    added
        .into_iter()
        .filter(|url| !removed.contains(url))
        .partition_map(|url| match github_repo(url) {
            Some(repo) => Either::Left(repo),
            None => Either::Right(url),
        })
}

/// Reads the link from a list entry such as
/// `- [name](https://github.com/owner/repo) - description.`.
fn entry_url(line: &str) -> Option<&str> {
    let entry = line.trim_start().strip_prefix("- [")?;
    let (_name, rest) = entry.split_once("](")?;
    let (url, _description) = rest.split_once(')')?;
    Some(url)
}

/// Reads the owner and repo from a link such as `https://github.com/owner/repo`,
/// ignoring anything after them like `/tree/main` or `#section`.
fn github_repo(url: &str) -> Option<(&str, &str)> {
    let path = url.strip_prefix("https://github.com/")?;
    let mut parts = path.split(['/', '#']);
    Some((parts.next()?, parts.next()?))
}

struct RepoResult {
    name: String,
    age_days: i64,
    active_days: i64,
    stars: u32,
    problems: Vec<String>,
}

/// Measures how old and how active a repository was when the pull request was
/// opened, and lists the rules it breaks.
async fn check_repo(
    github: &Octocrab,
    owner: &str,
    repo: &str,
    opened_at: DateTime<Utc>,
) -> Result<RepoResult> {
    let repos = github.repos(owner, repo);
    let info = repos.get().await?;
    let name = info.full_name.context("repository has no name")?;
    let created_at = info.created_at.context("repository has no creation date")?;
    let stars = info
        .stargazers_count
        .context("repository has no star count")?;

    let first_commit = first_commit_date(&repos).await?;
    let started_at = first_commit.max(created_at);
    let age_days = (opened_at - started_at).num_days();
    let active_days = count_active_days(&repos, opened_at).await?;
    let problems = list_problems(age_days, active_days, stars);
    Ok(RepoResult {
        name,
        age_days,
        active_days,
        stars,
        problems,
    })
}

/// Finds the date of the first commit. The commits API lists newest first, so
/// with one commit per page the last page holds the first commit. GitHub leaves
/// out the last page link when there's only one page. This takes two requests
/// however long the history is.
async fn first_commit_date(repos: &RepoHandler<'_>) -> Result<DateTime<Utc>> {
    let newest = repos.list_commits().per_page(1).send().await?;
    let Some(last_page) = newest.number_of_pages() else {
        let only_commit = newest.items.first().context("repository has no commits")?;
        return commit_date(only_commit);
    };

    let oldest = repos
        .list_commits()
        .per_page(1)
        .page(last_page)
        .send()
        .await?;
    let first_commit = oldest
        .items
        .first()
        .context("last page of commits is empty")?;
    commit_date(first_commit)
}

/// Counts the different days with commits up to `until`, from one page of the
/// 100 most recent commits. That is plenty to reach `MIN_ACTIVE_DAYS` for any
/// repository that has been worked on over time.
async fn count_active_days(repos: &RepoHandler<'_>, until: DateTime<Utc>) -> Result<i64> {
    let page = repos
        .list_commits()
        .until(until)
        .per_page(100)
        .send()
        .await?;
    let mut days = HashSet::new();
    for commit in &page.items {
        days.insert(commit_date(commit)?.date_naive());
    }
    Ok(days.len().try_into()?)
}

/// Returns when a commit was committed. That date changes when a commit is
/// rebased, unlike the author date, so it better reflects when work landed.
fn commit_date(commit: &RepoCommit) -> Result<DateTime<Utc>> {
    let committer = commit
        .commit
        .committer
        .as_ref()
        .context("commit has no committer")?;
    committer.date.context("commit has no committer date")
}

/// Describes each rule the project breaks, or returns an empty list if it
/// passes. Enough stars passes the project whatever its age or activity.
fn list_problems(age_days: i64, active_days: i64, stars: u32) -> Vec<String> {
    let mut problems = Vec::new();
    if stars >= MIN_STARS {
        return problems;
    }
    if age_days < MIN_AGE_DAYS {
        problems.push(format!("only {} old", plural(age_days, "day")));
    }
    if active_days < MIN_ACTIVE_DAYS {
        problems.push(format!("commits on only {}", plural(active_days, "day")));
    }
    problems
}

/// Writes the results as a Markdown table, with the rule underneath so a
/// contributor whose check failed can see what's expected. Links that aren't
/// on GitHub get a row asking for a manual check. In Actions it goes to the job
/// summary; locally it is printed.
fn write_summary(results: &[RepoResult], others: &[&str]) -> Result<()> {
    let checked = results.iter().map(summary_row);
    let unchecked = others
        .iter()
        .map(|url| format!("| {url} | | | | ⚠️ not on GitHub, check by hand |\n"));
    let rows: String = checked.chain(unchecked).collect();
    let summary = formatdoc! {"
        ### New entries

        | Project | Age | Days with commits | Stars | Result |
        | --- | --- | --- | --- | --- |
        {rows}
        Let it bake: projects need to be at least {MIN_AGE_DAYS} days old, with commits on at least
        {MIN_ACTIVE_DAYS} different days, when the pull request is opened, or have at
        least {MIN_STARS} stars. A maintainer can still merge an exception.
    "};

    match env::var("GITHUB_STEP_SUMMARY") {
        Ok(path) => fs::write(path, summary)?,
        Err(_) => print!("{summary}"),
    }
    Ok(())
}

/// Formats one result as a Markdown table row.
fn summary_row(result: &RepoResult) -> String {
    let link = format!("[{0}](https://github.com/{0})", result.name);
    let age = plural(result.age_days, "day");
    let verdict = match result.problems.as_slice() {
        [] => "✅".to_string(),
        problems => format!("❌ {}", problems.join("; ")),
    };
    format!(
        "| {link} | {age} | {} | {} | {verdict} |\n",
        result.active_days, result.stars
    )
}

/// Fails with one error per project that broke a rule, so the reasons show as
/// annotations on the check. Does nothing if every project passed.
fn fail_if_any_problems(results: &[RepoResult]) -> Result<()> {
    let failed: Vec<_> = results
        .iter()
        .filter(|result| !result.problems.is_empty())
        .collect();
    if failed.is_empty() {
        return Ok(());
    }
    for result in &failed {
        let problems = result.problems.join("; ");
        println!("::error::{} isn't quite baked yet: {problems}", result.name);
    }
    let count = plural(failed.len().try_into()?, "project");
    bail!("{count} not quite baked yet");
}

/// Formats a count with a word, adding "s" unless the count is 1: `1 day`, `5 days`.
fn plural(count: i64, word: &str) -> String {
    let suffix = if count == 1 { "" } else { "s" };
    format!("{count} {word}{suffix}")
}

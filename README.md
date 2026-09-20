<!--lint disable awesome-git-repo-age-->

# Awesome Ratatui [![Awesome](https://awesome.re/badge-flat2.svg)](https://github.com/sindresorhus/awesome)

[<img src="https://github.com/ratatui.png" align="right" width="100">](https://ratatui.rs)

Here you will find a list of TUI crates and applications that are made for or using [`ratatui`](https://crates.io/crates/ratatui) and [`tui`](https://crates.io/crates/tui).

<!--lint disable awesome-toc-->

## Contents

- [📦 Libraries](#-libraries)
  - [🏗️ Frameworks](#%EF%B8%8F-frameworks)
  - [🧩 Widgets](#-widgets)
  - [🔧 Utilities](#-utilities)
  - [🔗 Bindings](#-bindings)
- [💻 Apps](#-apps)
  - [⌨️ Development Tools](#%EF%B8%8F-development-tools)
    - [Source Control and Collaboration](#source-control-and-collaboration)
    - [Code Search, Editing, and Review](#code-search-editing-and-review)
    - [APIs, Databases, Build, and Debugging](#apis-databases-build-and-debugging)
  - [🤖 AI and Agents](#-ai-and-agents)
  - [📁 Files, Data, and Documents](#-files-data-and-documents)
  - [🧰 Terminal Workflow](#-terminal-workflow)
  - [🌐 Networking and Internet](#-networking-and-internet)
    - [Network Operations and Infrastructure](#network-operations-and-infrastructure)
    - [Remote Access, APIs, and File Transfer](#remote-access-apis-and-file-transfer)
    - [Communications and Social](#communications-and-social)
  - [👨‍💻 System Administration](#-system-administration)
    - [Monitoring, Diagnostics, and Logs](#monitoring-diagnostics-and-logs)
    - [Containers and Orchestration](#containers-and-orchestration)
    - [OS, Storage, and Package Management](#os-storage-and-package-management)
    - [Batch, Database, and Cluster Operations](#batch-database-and-cluster-operations)
  - [🔌 Hardware and Embedded](#-hardware-and-embedded)
  - [🔐 Security and Identity](#-security-and-identity)
  - [📝 Productivity and Planning](#-productivity-and-planning)
    - [Tasks, Projects, and Calendars](#tasks-projects-and-calendars)
    - [Notes and Journaling](#notes-and-journaling)
    - [Finance and Markets](#finance-and-markets)
    - [Focus, Habits, and Time](#focus-habits-and-time)
  - [📚 Reading and Learning](#-reading-and-learning)
  - [🎵 Music and Media](#-music-and-media)
    - [Music and Audio](#music-and-audio)
    - [Books, Video, and Creative Media](#books-video-and-creative-media)
  - [🎮 Games and Entertainment](#-games-and-entertainment)
  - [🔬 Science, Math, and Exploration](#-science-math-and-exploration)

Aside from those listed here, many other apps and libraries can be easily be found via the reverse dependencies on crates.io and GitHub:

- <https://crates.io/crates/ratatui/reverse_dependencies>
- <https://crates.io/crates/tui/reverse_dependencies>
- <https://github.com/ratatui/ratatui/network/dependents>
- <https://github.com/fdehau/tui-rs/network/dependents?package_id=UGFja2FnZS0zMjE3MzkzMDMx>

## 📦 Libraries

### 🏗️ Frameworks

- [bevy_ratatui_camera](https://github.com/cxreiff/bevy_ratatui_camera) - A bevy plugin for rendering your bevy app to the terminal using ratatui.
- [burn](https://github.com/tracel-ai/burn) - Comprehensive Deep Learning framework in Rust.
- [crepuscularity](https://github.com/tschk/crepuscularity) - One UI codebase for desktop, web, mobile, terminal, browser extensions, and embedded devices. Write React JSX or our lightweight DSL, get GPUI, Ratatui, SwiftUI, LVGL, and more. Batteries included.
- [dumo](https://github.com/iddey/dumo) - An embedded-graphics backend that is built on [mplusfonts](https://github.com/iddey/mplusfonts) and has kanji support.
- [egui-ratatui](https://github.com/gold-silver-copper/egui_ratatui) - A ratatui backend that is also an egui widget. Deploy on web with WebAssembly or ship natively with bevy, macroquad, or eframe.
- [mousefood](https://github.com/ratatui/mousefood) - An embedded-graphics backend for Ratatui.
- [raclettui](https://github.com/ishrut/raclettui) - A wayland layer shell window implementing the ratatui backend with cpu and wgpu rendering.
- [rat-salsa](https://github.com/thscharler/rat-salsa) - An event-queue for ratatui with tasks, timers, application events, focus handling, dialog windows.
- [ratatuefi](https://github.com/sermuns/ratatuefi) - A Ratatui backend for drawing terminal UIs in pre-boot UEFI environments.
- [ratatui-kit](https://github.com/yexiyue/ratatui-kit) - A React-style component framework for Ratatui with hooks, routing, async state, input layers, and reusable components.
- [ratatui-minecraft](https://github.com/janTatesa/ratatui-minecraft) - A Ratatui backend for rendering terminal UIs inside Minecraft via [valence-screens](https://github.com/White-145/valence-screens).
- [ratatui-uefi](https://github.com/reubeno/tui-uefi) - A Ratatui backend for drawing terminal UIs in pre-boot UEFI environments.
- [ratatui-wgpu](https://github.com/Jesterhearts/ratatui-wgpu) - A wgpu based rendering backend for ratatui.
- [ratzilla](https://github.com/ratatui/ratzilla) - Build terminal-themed web applications with Ratatui and WebAssembly.
- [rlt](https://crates.io/crates/rlt) - A universal load testing framework for Rust, with real-time tui support.
- [schemaui](https://github.com/YuniqueUnic/schemaui) - Turn JSON Schemas into TUIs and web UIs with real-time validation.
- [soft_ratatui](https://github.com/gold-silver-copper/soft_ratatui) - A software rendering backend for ratatui. No GPU required. TUI everywhere.
- [tui-react](https://crates.io/crates/tui-react) - TUI widgets using a react-like paradigm.
- [tui-realm](https://crates.io/crates/tuirealm) - A ratatui framework inspired by Elm and React.
- [webatui](https://github.com/TylerBloom/webatui) - An integration between the Yew and Ratatui crates for making TUI-themed WebAssembly webapps.
- [widgetui](https://crates.io/crates/widgetui) - A bevy-like widget system for ratatui and crossterm.
- [xnano](https://github.com/hsaeed3/xnano) - A declarative terminal framework for Python built on ratatui & ratzilla.

### 🧩 Widgets

- [edtui](https://github.com/preiter93/edtui) - A Vim-inspired terminal editor.
- [hyperrat](https://crates.io/crates/hyperrat) - Clickable terminal links.
- [malevich](https://crates.io/crates/malevich) - A plotting widget: line, scatter, bar, histogram, heatmap, box plot, violin, and more, with automatic axes and millions of points.
- [rat-widget](https://crates.io/crates/rat-widget) - Widgets for data-input (text-input, date- and number-input, text-area, checkbox, choice, radiobutton, slider, calendar), structural widgets (view, split, tabbed, multi-page), a table widget for large data-sets, a file-dialog, a menubar+sub-menus, a status-bar and some more. With builtin crossterm event-handling and focus-handling.
- [ratatui-cheese](https://crates.io/crates/ratatui-cheese) - Bubbletea-inspired widgets, including spinner, help, tree, paginator, and list.
- [ratatui-code-editor](https://github.com/vipmax/ratatui-code-editor) - A code editor with syntax highlighting powered by tree-sitter.
- [ratatui-comfy-tabs](https://crates.io/crates/ratatui-comfy-tabs) - A feature-rich tab navigation for TUI. Highly customizable.
- [ratatui-comfy-toaster](https://crates.io/crates/ratatui-comfy-toaster) - An advanced toast-notification engine for terminal UI applications.
- [ratatui-explorer](https://github.com/tatounee/ratatui-explorer) - A simple file-explorer library.
- [ratatui-form](https://github.com/DavidLiedle/ratatui-form) - A form library.
- [ratatui-fretboard](https://crates.io/crates/ratatui-fretboard) - Displays musical note positions on a fretboard.
- [ratatui-image](https://crates.io/crates/ratatui-image) - Displays images using Sixel graphics or Unicode half-blocks.
- [ratatui-markdown](https://github.com/celestia-island/ratatui-markdown) - A Rust library providing markdown rendering, Mermaid diagrams, syntax highlighting, collapsible JSON/TOML tree views, and a rich hybrid scroll system.
- [ratatui-splash-screen](https://github.com/orhun/ratatui-splash-screen) - Turns any image into a splash screen.
- [ratatui-stacked-bar](https://github.com/zeqianli/ratatui-stacked-bar) - A stacked area chart.
- [ratatui-textarea](https://crates.io/crates/ratatui-textarea) - A simple yet powerful editor; fork of `tui-textarea`.
- [ratatui-toaster](https://crates.io/crates/ratatui-toaster) - An extremely lightweight toast engine.
- [ratatui-tournament](https://github.com/philipgreat/ratatui-tournament) - A widget for rendering single-elimination tournament brackets in the terminal.
- [ratatui-wireframe](https://crates.io/crates/ratatui-wireframe) -  A widget for rendering and rotating 3D wireframe models.
- [ratiform](https://crates.io/crates/ratiform) - A stateful form widget with typed field identifiers, so your data model stays your own, not the library's.
- [term-rustdoc](https://github.com/zjp-CN/term-rustdoc) - A TUI for Rust docs that aims to improve the UX on tree view and generic code.
- [throbber-widgets-tui](https://crates.io/crates/throbber-widgets-tui) - A widget that displays throbber.
- [tui-additions](https://crates.io/crates/tui-additions) - Additions to the rust tui crate.
- [tui-bar-graph](https://crates.io/crates/tui-bar-graph) - A bar graph widget for Ratatui.
- [tui-big-text](https://crates.io/crates/tui-big-text) - Displays big text using the `font8x8` crate.
- [tui-box-text](https://crates.io/crates/tui-box-text) - Displays text surrounded by a box.
- [tui-cards](https://crates.io/crates/tui-cards) - Widgets for displaying cards.
- [tui-equalizer](https://crates.io/crates/tui-equalizer) - An equalizer widget for Ratatui.
- [tui-checkbox](https://crates.io/crates/tui-checkbox) - A customizable checkbox.
- [tui-dialog](https://crates.io/crates/tui-dialog) - A widget for entering a single line of text in a dialog.
- [tui-globe](https://github.com/d10n/tui-globe) - A 3D globe widget rendered in the terminal with Braille characters.
- [tui-input](https://crates.io/crates/tui-input) - A headless input library for TUI apps.
- [tui-logger](https://crates.io/crates/tui-logger) - A logger with a smart log viewer.
- [tui-menu](https://github.com/shuoli84/tui-menu) - A menu component.
- [tui-nodes](https://crates.io/crates/tui-nodes) - Visualize node graphs.
- [tui-overlay](https://crates.io/crates/tui-overlay) - A composable overlay widget with drawers, modals, popovers, and toasts from a single configurable primitive.
- [tui-piechart](https://crates.io/crates/tui-piechart) - A configurable, colorful piechart widget that comes in standard and high resolution.
- [tui-popup](https://crates.io/crates/tui-popup) - A popup component.
- [tui-prompts](https://crates.io/crates/tui-prompts) - A library for building interactive prompts.
- [tui-qrcode](https://crates.io/crates/tui-qrcode) - A QR code widget for Ratatui.
- [tui-rain](https://github.com/levilutz/tui-rain) - A widget to generate various rain effects.
- [tui-scrollbar](https://crates.io/crates/tui-scrollbar) - A scrollbar widget for Ratatui.
- [tui-scrollview](https://crates.io/crates/tui-scrollview) - A container that provides a scrolling view at a larger area.
- [tui-shimmer](https://github.com/vinhnx/tui-shimmer) - A shimmer text effect.
- [tui-skeleton](https://crates.io/crates/tui-skeleton) - A library of placeholder widgets that pulse, sweep, or shimmer while your content loads.
- [tui-slider](https://crates.io/crates/tui-slider) - A highly customizable slider widget for both horizontal and vertical orientations.
- [tui-tabs](https://crates.io/crates/tui-tabs) - A tab navigation widget with individually bordered boxes and rounded corners.
- [tui-term](https://crates.io/crates/tui-term) - Embeds interactive terminal sessions.
- [tui-textarea](https://crates.io/crates/tui-textarea) - A simple yet powerful terminal text editor.
- [tui-tree-widget](https://crates.io/crates/tui-tree-widget) - A tree view.
- [tui-widget-list](https://crates.io/crates/tui-widget-list) - A versatile list implementation.

### 🔧 Utilities

- [ansi-to-tui](https://crates.io/crates/ansi-to-tui) - A library to convert ansi color coded text into `ratatui::text::Text`.
- [bevy_ratatui](https://github.com/ratatui/bevy_ratatui) - A Rust crate to use Ratatui in a Bevy App.
- [color-to-tui](https://crates.io/crates/color-to-tui) - Parse colors and convert them to `ratatui::style::Colors`.
- [coolor](https://github.com/Canop/coolor) - Tiny color conversion library for TUI application builders.
- [ggsci-ratatui](https://github.com/nanxstats/ggsci-rs) - Scientific and sci-fi color palettes from ggsci as Ratatui colors and styles, in truecolor or ANSI-256 mode.
- [opaline](https://crates.io/crates/opaline) - Token-based theme engine for Ratatui with gradients, 20 builtin themes, user theme discovery, and a reusable theme selector widget.
- [ratatui-garnish](https://github.com/franklaranja/ratatui-garnish) - A powerful composition system for Ratatui widgets.
- [ratatui-input-manager](https://crates.io/crates/ratatui-input-manager) - A macro for creating declarative update handlers in Elm style apps, supporting crossterm, termion and termwiz.
- [ratatui-interact](https://github.com/Brainwires/ratatui-interact) - Interactive TUI components for Ratatui with focus management and mouse support.
- [tachyonfx](https://github.com/ratatui/tachyonfx) - An animation and visual-effects library for Ratatui applications.
- [terminput](https://crates.io/crates/terminput) - An abstraction over various backends that provide input events.
- [termprofile](https://github.com/aschey/termprofile) - Detect and handle terminal color/styling support. Supports converting Ratatui color and style objects.
- [tui-pantry](https://crates.io/crates/tui-pantry) - Component-driven development tool for ratatui widgets, similar to Storybook.
- [tui-syntax-highlight](https://github.com/aschey/tui-syntax-highlight) - Syntax highlighting for code blocks.

### 🔗 Bindings

- [ex_ratatui](https://github.com/mcass19/ex_ratatui) - Elixir bindings for ratatui.
- [jatatui](https://github.com/oyvindberg/jatatui) - A Java port of ratatui.
- [pyratatui](https://github.com/programmersd21/pyratatui) - Python bindings for Ratatui powered by PyO3, featuring async support, many widgets, and advanced terminal animations via TachyonFX.
- [ratatui-ffi](https://github.com/holo-q/ratatui-ffi) - Foreign Function Interface bindings for Ratatui.
- [ratatui-go](https://github.com/holo-q/ratatui-go) - Go bindings for ratatui.
- [ratatui-py](https://github.com/holo-q/ratatui-py) - Python bindings for ratatui.
- [ratatui-ts](https://github.com/holo-q/ratatui-ts) - TypeScript bindings for ratatui.
- [Ratatui.cs](https://github.com/holo-q/Ratatui.cs) - C# bindings for ratatui.
- [ratatui_ruby](https://sr.ht/~kerrick/ratatui_ruby/) - Ruby bindings for ratatui.

## 💻 Apps

### ⌨️ Development Tools

#### Source Control and Collaboration

- [blippy](https://github.com/AksharP5/blippy) - A keyboard-first TUI for GitHub issues and pull requests.
- [ComfyGit](https://github.com/comfy-home/ComfyGit) - All-In-One: Centralised multi-project management, Changelog generator, Version bumper & CLI tool introducing a new ComfyGitFlow.
- [deadbranch](https://github.com/armgabrielyan/deadbranch) - A TUI for cleaning stale Git branches safely.
- [drydock](https://github.com/yetidevworks/drydock) - A live dashboard for a fleet of Git repos, showing what's uncommitted, unpushed, and unreleased across all of them.
- [giff](https://github.com/bahdotsh/giff) - A TUI for Git diffs with interactive rebase support.
- [gimoji](https://github.com/zeenix/gimoji) - Makes it easy to add emojis to your Git commit messages.
- [git-time-machine](https://github.com/dinakars777/git-time-machine) - Visual Git reflog TUI for undoing Git mistakes.
- [Gitside](https://github.com/dev-bhaskar8/gitside) - A responsive, mouse-friendly Git source-control TUI for full terminals and narrow tmux panes.
- [gitu](https://github.com/altsem/gitu) - A TUI Git client inspired by Magit.
- [gitui](https://github.com/gitui-org/gitui) - Terminal UI for Git.
- [gitv](https://github.com/jayanaxhf/gitv) - A beautiful, feature-rich and performant terminal client for GitHub issues.
- [glim](https://github.com/junkdog/glim) - Monitor GitLab continuous integration and delivery pipelines and projects.
- [gwm](https://github.com/kbrdn1/gwm-cli) - A Git worktree manager: CLI and TUI in one binary, native libgit2, per-repo declarative bootstrap, and AI agent session tracking.
- [lazyjj](https://github.com/Cretezy/lazyjj) - A TUI for the Jujutsu version-control system.
- [Livediff](https://github.com/SoCkEt7/Livediff) - Real-time terminal file diff monitoring TUI.
- [nomad](https://github.com/JosephLai241/nomad) - Customizable next-gen tree command with Git integration and TUI.
- [rootle](https://github.com/rootledev/rootle) - A modal TUI for browsing GitHub and other source-control services with column views and syntax-highlighted previews.
- [serie](https://github.com/lusingander/serie) - A rich Git commit graph in your terminal.
- [wrkflw](https://github.com/bahdotsh/wrkflw) - A TUI for validating and executing GitHub Actions workflows locally.

#### Code Search, Editing, and Review

- [blogr](https://github.com/bahdotsh/blogr) - A terminal-based static site generator with a TUI editor for writing blog posts.
- [codemark](https://github.com/DanielCardonaRojas/codemark) - A semantic code bookmarking system for humans and agents.
- [igrep](https://github.com/konradsz/igrep) - Interactive Grep.
- [image-auditor](https://github.com/0franco/image-auditor) - A TUI for finding and fixing website image-performance issues such as layout shifts, lazy loading, WebP, and responsive images.
- [lingora-tui](https://github.com/nigeleke/lingora) - Browse, compare and validate Fluent i18n files.
- [Rustlens](https://github.com/yashksaini-coder/Rustlens) - A TUI Rust codebase inspector to browse functions, structs, enums, traits, and more.
- [regect](https://github.com/kloki/regect) - A regex101 like tool for the cli.
- [repgrep](https://github.com/acheronfail/repgrep) - An interactive replacer for ripgrep that makes it easy to find and replace across files on the command line.
- [rgx](https://github.com/brevity1swos/rgx) - A terminal regex debugger with real-time matching, 3 engines, capture group highlighting, replace mode, and plain-English explanations.
- [scooter](https://github.com/thomasschafer/scooter) - Interactive find and replace in the terminal.
- [Serpl](https://github.com/yassinebridi/serpl) - A simple terminal UI for search and replace, ala VS Code.
- [termi](https://github.com/tuna4ll/termi) - A modal terminal code editor.
- [VLE](https://github.com/tuffy/vle) - A lightweight text editor.
- [ygrep](https://github.com/yetidevworks/ygrep) - A fast, local, indexed code-search tool with a TUI for AI coding assistants.

#### APIs, Databases, Build, and Debugging

- [agx](https://github.com/brevity1swos/agx) - A step-through debugger for AI agent execution traces.
- [ATAC](https://github.com/Julien-cpsn/ATAC) - A feature-full TUI API client for your terminal.
- [BugStalker](https://github.com/godzie44/BugStalker) - Modern rust debugger for Linux x86-64.
- [cargo-selector](https://github.com/lusingander/cargo-selector) - Cargo subcommand to select and execute binary/example targets.
- [depot-rs](https://github.com/quietpigeon/depot-rs) - A TUI for managing crates.
- [deputui](https://github.com/twiddler/deputui) - Review and install JavaScript package updates from npm.
- [desed](https://github.com/SoptikHa2/desed) - Debugging tool for sed scripts.
- [envx](https://github.com/mikeleppane/envx) - Environment variable manager for developers, featuring an intuitive TUI.
- [get_blessed_rs](https://github.com/josueBarretogit/get_blessed_rs) - Get the best crates for your rust projects, curated by blessed.rs.
- [gobang](https://github.com/TaKO8Ki/gobang) - Cross-platform TUI database management tool.
- [Maelstrom](https://github.com/maelstrom-software/maelstrom) - A fast test runner that runs every test in its own container locally or distributed.
- [oha](https://github.com/hatoo/oha) - An HTTP load generator with a real-time terminal UI.
- [openapi-tui](https://github.com/zaghaghi/openapi-tui) - A terminal UI for browsing and calling APIs from OpenAPI specifications.
- [rainfrog](https://github.com/achristmascarl/rainfrog) - A database management TUI for Postgres.
- [ratifact](https://github.com/adolfousier/ratifact) - Track and manage build artifacts from multiple programming languages.
- [raygun](https://github.com/yetidevworks/raygun) - A terminal-based receiver for Spatie's Ray debugger, compatible with the Ray HTTP protocol used by PHP, Laravel, and Grav.
- [sabiql](https://github.com/riii111/sabiql) - Fast, driverless, Vim-first database TUI with safe editing and ER diagrams.
- [Surfpool](https://github.com/solana-foundation/surfpool) - A local-first Solana development environment with mainnet state, infrastructure as code and transaction debugging.
- [TaskUI](https://github.com/thmshmm/taskui) - Simple Terminal UI for Task / taskfile.dev.
- [tongo](https://github.com/drewzemke/tongo) - A TUI for MongoDB.
- [tracexec](https://github.com/kxxt/tracexec) - A terminal tool for tracing which programs and commands are executed.
- [wireman](https://github.com/preiter93/wireman) - A terminal client for calling gRPC services.

### 🤖 AI and Agents

- [agent-console](https://github.com/buhuipao/agent-console) - A local dashboard for Codex and Claude Code.
- [amtr](https://github.com/arian-shamaei/anthropometer) - A btop-style TUI that renders a Claude Code session's live context window as a memory map, with tool and file traffic, cache economics, and a compiled PDF report.
- [OpenBitFun](https://github.com/GCWing/OpenBitFun) - An open-source desktop workspace for general-purpose AI agents.
- [bosun](https://github.com/yetidevworks/bosun) - A tmux-native TUI for orchestrating AI coding agent sessions (Claude Code, Codex) with live previews and per-session state.
- [claudectl](https://github.com/mercurialsolo/claudectl) - Mission control for multiple Claude Code sessions with live dashboard, cost tracking, and budget enforcement.
- [commandOK](https://github.com/64bit/commandOK) - A Spotlight-like terminal command generator powered by major large-language-model providers.
- [crmux](https://github.com/maedana/crmux) - A TUI viewer for monitoring and managing multiple Claude Code sessions in tmux.
- [Forge](https://github.com/NorviaLabs/forge) - An AI coding agent that unifies an agent, code editor, and shell in a single keyboard-driven terminal workspace.
- [gmsg](https://github.com/olorikendrick/gmsg) - Generate, edit, and commit AI-powered Git commit messages from a single TUI.
- [hedos](https://github.com/theiskaa/hedos) - A terminal shelf for the local AI models already on your machine, with a built-in OpenAI-compatible gateway.
- [ilmari](https://github.com/bnomei/ilmari) - A tmux popup dashboard for monitoring AI coding agents.
- [iris](https://github.com/itzenata/iris-tui) - Live supervisor for every active Claude Code session - status, tokens, estimated cost, and one-pane approval of tool calls.
- [LimitDeck](https://github.com/rockythink/limitdeck) - A compact, privacy-safe terminal dashboard for AI coding subscription limits.
- [llmtrim](https://github.com/fkiene/llmtrim) - A local proxy that compresses large-language-model API requests to reduce token costs, with a dashboard for per-source costs and context usage.
- [Martty](https://github.com/openma-ai/Martty) - An extensible Rust/ratatui terminal client for DeepSeek Harness and ACP-compatible coding agents, with plugins, tools, subagents, and durable sessions.
- [models](https://github.com/reyamira/models) - A TUI for browsing AI models, benchmarks, and coding agents.
- [nereid](https://github.com/bnomei/nereid) - Create and explore Mermaid diagrams with AI agents through a terminal UI and Model Context Protocol server.
- [Oatmeal](https://github.com/dustinblackman/oatmeal) - Terminal UI to chat with large language models (LLM) using different model backends, and integrations with your favourite editors!
- [opencode stats](https://github.com/Cateds/opencode-stats) - A terminal dashboard for OpenCode usage statistics and cost breakdowns.
- [opencrabs](https://github.com/adolfousier/opencrabs) - An all-in-one AI agent for the terminal with TUI, CLI, and daemon modes.
- [partly-claudy](https://github.com/taho-inc/partly-claudy) - Terminal view of Claude status page (status.claude.com).
- [patent](https://github.com/r14dd/patent) - A prior-art search for developer-tool ideas with an AI-generated verdict.
- [pixtuoid](https://github.com/IvanWng97/pixtuoid) - Live pixel-art office for AI coding agents.
- [Proqi](https://github.com/oborchers/proqi) - A terminal-native prompt composer for developers working with coding agents.
- [raymon](https://github.com/bnomei/raymon) - A terminal UI and Model Context Protocol (MCP) server for receiving and searching Ray-style debug logs.
- [Reeve](https://github.com/Dancode-188/reeve) - A terminal cockpit for AI agents: watch a run live, score it, and step in when it goes sideways.
- [savras](https://github.com/Marijusaj/savras) - A side panel that sees every Claude Code session you have running: which are waiting on you, working, or done, with their PRs and context used.
- [Stakpak](https://github.com/stakpak/agent) - AI DevOps agent to help you secure, deploy, and maintain production-ready infrastructure.
- [SynapsCLI](https://github.com/HaseebKhalid1507/SynapsCLI) - Lightning fast terminal native agent harness with tools, extensions and subagents. 15MB, 2ms boot.
- [tenere](https://github.com/pythops/tenere) - A terminal chat interface for large language models, written in Rust.
- [thurbox](https://github.com/Thurbeen/thurbox) - A TUI orchestrator for running multiple AI coding agents (Claude Code, Codex, and others) in persistent tmux sessions.
- [toktop](https://github.com/htin1/toktop) - A terminal dashboard for monitoring OpenAI and Anthropic token usage and costs.
- [Tree Ring Memory](https://github.com/TerminallyLazy/Tree-Ring-Memory) - A local-first memory system for AI agents with SQLite full-text search, auditing, forgetting, consolidation, and a Ratatui operator console.
- [VT Code](https://github.com/vinhnx/vtcode) - An open-source Rust coding agent for the terminal.
- [Yardlet](https://github.com/zzunkie/yardlet) - A local AI workbench that turns intent into a verified task queue and drives your installed Claude Code or Codex CLIs as interchangeable workers.

### 📁 Files, Data, and Documents

- [binsider](https://github.com/orhun/binsider) - A TUI for analyzing binary files.
- [columbus](https://github.com/sivaprakashkrp/columbus) - A GUI-like TUI file explorer.
- [comhad](https://github.com/Eoin-McMahon/Comhad) - A ranger-style terminal browser for S3, with previews, background transfers, and non-destructive sync.
- [csvlens](https://github.com/YS-L/csvlens) - Command line csv viewer.
- [ddv](https://github.com/lusingander/ddv) - Terminal DynamoDB viewer.
- [dead-ringer](https://github.com/ztroop/dead-ringer) - A binary diff tool for comparing files in hexadecimal and ASCII.
- [diskonaut](https://github.com/imsnif/diskonaut) - Terminal-based disk space navigator.
- [doxx](https://github.com/bgreenwell/doxx) - Document viewer for Microsoft Word files.
- [edamame](https://github.com/mijowi/edamame) - A Terminal Markdown editor that stays rendered while you edit.
- [FileSSH](https://github.com/JayanAXHF/filessh) - A TUI-based file explorer for remote servers.
- [flerp](https://github.com/Huseynteymurzade28/flerp) - A TUI for exploring and analyzing text files, PDFs and images.
- [hexhog](https://github.com/DVDTSB/hexhog) - TUI Hex Editor/Viewer.
- [joshuto](https://github.com/kamiyaa/joshuto) - Ranger-like terminal file manager written in Rust.
- [md-tui](https://github.com/henriklovhaug/md-tui) - Markdown renderer in the terminal.
- [otree](https://github.com/fioncat/otree) - A command line tool to view objects (JSON/YAML/TOML) in TUI tree widget.
- [rat-commander](https://github.com/dividebysandwich/rat-commander) - A fully-featured modern spiritual successor to Midnight-Commander with truecolor support and built-in process- and disk-explorer.
- [rdn](https://github.com/apatrushev/rdn) - Rust port of well known old Dos Navigator.
- [sheetsui](https://github.com/zaphar/sheetsui) - A terminal based spreadsheet application.
- [stu](https://github.com/lusingander/stu) - A TUI for AWS S3.
- [tabiew](https://github.com/shshemi/tabiew) - A lightweight TUI app to view and query CSV files.
- [traceview](https://github.com/javaLux/traceview) - Tracing and viewing your files and resource landscape.
- [TSHTS](https://github.com/SamuelSchlesinger/tshts) - A terminal based spreadsheet application.
- [vib](https://github.com/ayanchavand/vib) - A terminal file browser with LocalSend built in, for managing, organizing and transferring files across devices.
- [wiper](https://github.com/ikebastuz/wiper) - Disk space analyzer and cleanup tool.
- [xan](https://github.com/medialab/xan) - A terminal tool for processing CSV files.
- [xplr](https://github.com/sayanarijit/xplr) - Hackable, minimal, and fast TUI file explorer.
- [Yazi](https://github.com/sxyazi/yazi) - Blazing fast terminal file manager written in Rust, based on async I/O.

### 🧰 Terminal Workflow

- [absorb](https://github.com/kloki/absorb) - Quickly read a file without moving your eyes.
- [atuin](https://github.com/atuinsh/atuin) - A shell history manager with fast search and optional sync.
- [exabind](https://github.com/junkdog/exabind) - An animated TUI for viewing KDE shortcuts.
- [flyline](https://github.com/HalFrgrd/flyline) - A Bash plugin TUI for an enhanced command line writing experience.
- [fsel](https://github.com/Mjoyufull/fsel) - A TUI app launcher and fuzzy finder for GNU/Linux and BSD.
- [fzf-make](https://github.com/kyu08/fzf-make) - A command line tool that executes make target using fuzzy finder with preview window.
- [hwatch](https://github.com/blacknon/hwatch) - Alternative watch command with command history and diffs.
- [kbt](https://github.com/bloznelis/kbt) - Keyboard tester in terminal.
- [matchmaker](https://github.com/Squirreljetpack/matchmaker) - A fast, configurable fuzzy searcher for terminal data.
- [material](https://github.com/azorng/material) - A material design color palette for the terminal.
- [dekit](https://github.com/pvolok/dekit) - Run multiple commands in parallel and shows output of each command separately.
- [p2pmux](https://github.com/pelazas/p2pmux) - A peer-to-peer terminal multiplexer for sharing sessions across machines.
- [sigye](https://github.com/am2rican5/sigye) - A terminal clock with FIGlet fonts, customizable themes, and animated backgrounds.
- [snipt](https://github.com/snipt/snipt) - A text snippet expansion tool with a TUI for managing snippets.
- [splashboard](https://github.com/unhappychoice/splashboard) - A customizable terminal splash rendered on shell startup or directory updates.
- [television](https://github.com/alexpasmantier/television) - A blazingly fast general purpose fuzzy finder for your terminal.
- [trex](https://github.com/blackopsrepl/trex) - A fast tmux session manager with fuzzy finding, per session stats and AI Agent tracking.

### 🌐 Networking and Internet

#### Network Operations and Infrastructure

- [AdGuardian-Term](https://github.com/Lissy93/AdGuardian-Term) - Real-time traffic monitoring and statistics for AdGuard Home.
- [adsb_deku/radar](https://github.com/wcampbell0x2a/adsb_deku#radar-tui) - A terminal radar for displaying aircraft positions from Automatic Dependent Surveillance–Broadcast (ADS-B) data.
- [bandwhich](https://github.com/imsnif/bandwhich) - Displays network utilization by process.
- [discovery-rs](https://github.com/JustPretender/discovery-rs) - A TUI for discovering services on your local network.
- [dnsglobe](https://github.com/514-labs/dnsglobe) - Global DNS propagation checker querying 34 resolvers worldwide, with a world map.
- [dsnitch](https://github.com/infomaniac777/dsnitch) - Real-time network and DNS egress inspector TUI for Docker containers powered by eBPF.
- [FlowLens](https://github.com/power4j/flowlens) - A cross-platform network traffic analyzer with process, IP, and outbound-domain attribution.
- [gping](https://github.com/orf/gping/) - Ping tool with a graph.
- [impala](https://github.com/pythops/impala) - TUI for managing wifi on Linux.
- [LazyMQTT](https://github.com/ScottFelder/lazymqtt) - A keyboard-driven terminal client for browsing and publishing messages over the MQTT messaging protocol.
- [mqttui](https://github.com/EdJoPaTo/mqttui) - A terminal client for subscribing to and publishing messages over MQTT.
- [mullvad-tui](https://github.com/d10n/mullvad-tui) - A TUI for Mullvad VPN.
- [netscanner](https://github.com/Chleba/netscanner) - Network scanning tool.
- [nordvpn-tui](https://github.com/Degra02/nordvpn-tui) - A TUI for NordVPN.
- [oryx](https://github.com/pythops/oryx) - A TUI for sniffing network traffic using eBPF.
- [rustnet](https://github.com/domcyrus/rustnet) - A cross-platform network monitoring tool with deep packet inspection.
- [sensor-vision](https://github.com/jcfromsiberia/sensor-vision) - TUI Client for TeamViewer IoT MQTT API for managing IoT Sensors and Metrics.
- [streamtop](https://github.com/Jorji49/streamtop) - A terminal monitor for live video streams with real-time health checks and metrics.
- [trippy](https://github.com/fujiapple852/trippy) - Network diagnostic tool.
- [unifly](https://github.com/hyperb1iss/unifly) - CLI and TUI for managing Ubiquiti UniFi network controllers with an 8-screen dashboard, live traffic charts, and dual-API coverage.
- [vortix](https://github.com/Harry-kp/vortix) - Terminal UI for WireGuard and OpenVPN with real-time telemetry, leak detection, and kill switch.
- [wiretui](https://github.com/robin-thoene/wiretui) - A minimal keyboard-driven TUI to manage WireGuard VPN connections.
- [yscan](https://github.com/yetidevworks/yscan) - A TUI-first network scanner with ARP, mDNS, and SSDP discovery.
- [ytunnel](https://github.com/yetidevworks/ytunnel) - A TUI-first CLI for managing Cloudflare Tunnels with custom domains.

#### Remote Access, APIs, and File Transfer

- [jdtui](https://github.com/rylos/jdtui) - A TUI for JDownloader 2 over the My.JDownloader API: downloads, link grabber, accounts and settings, from anywhere.
- [JocalSend](https://git.kittencollective.com/nebkor/joecalsend) - Peer to peer local file and data transfer, compatible with [LocalSend](https://github.com/localsend/localsend)
- [lazy-etherscan](https://github.com/woxjro/lazy-etherscan) - A Simple Terminal UI for the Ethereum Blockchain Explorer.
- [nyaa](https://github.com/Beastwick18/nyaa) - A nyaa.si tui tool for browsing and downloading torrents.
- [purple](https://github.com/erickochen/purple) - TUI SSH config manager & launcher with fuzzy search, tags, cloud provider sync, tunnels and command snippets for server management.
- [rustmission](https://github.com/intuis/rustmission) - TUI for the Transmission daemon.
- [slumber](https://github.com/LucasPickering/slumber) - Terminal-based HTTP/REST client.
- [ssh-list](https://github.com/akinoiro/ssh-list) - SSH connection manager.
- [terminusdm](https://github.com/sumoduduk/terminusdm) - Cross Platform Terminal Download Manager.
- [TermiRs](https://github.com/caelansar/termirs) - A modern, async SSH terminal client.
- [termscp](https://github.com/veeso/termscp) - A feature rich terminal UI file transfer and explorer with support for SCP/SFTP/FTP/S3/SMB.
- [traxor](https://github.com/kristoferssolo/traxor) - A TUI for managing Transmission torrents.
- [vincenzo](https://github.com/gabrieldemian/vincenzo) - A bittorrent client for the terminal with vim-like keybindings.

#### Communications and Social

- [Chat-gRPC](https://github.com/Atheer2104/chat-grpc) - A Real-time Chat Microservice built in Rust using gRPC, including a TUI client.
- [concord](https://github.com/chojs23/concord) - A TUI client for Discord.
- [hnr](https://github.com/prasanthj/hnr) - A terminal UI for Hacker News — browse feeds, read threaded comments, vote, reply, search, and bookmark.
- [iamb](https://github.com/ulyssa/iamb) - A matrix chat client with vim keybindings.
- [lobtui](https://github.com/pythops/lobtui) - TUI for lobste.rs website.
- [mxr](https://github.com/planetaryescape/mxr) - Local-first email client with Vim-style navigation, multi-account sync, and full-text search.
- [nostui](https://github.com/akiomik/nostui) - A TUI client for Nostr.
- [omaro](https://github.com/Rolv-Apneseth/omaro) - TUI for the lobste.rs website.
- [rvIRC](https://github.com/KaraZajac/rvIRC) - A TUI client for IRC.
- [termchat](https://github.com/lemunozm/termchat) - Terminal chat over a local network with video streaming and file transfer.
- [tgt](https://github.com/FedericoBruzzone/tgt) - A TUI for Telegram written in Rust.
- [tincan](https://github.com/bilalyazicioglu/tincan-cli) - Peer-to-peer voice and text chat with no server.
- [tsuchita](https://github.com/kamiyaa/tsuchita) - A client-server notification center for desktop notifications on Linux.
- [tuisky](https://github.com/sugyan/tuisky) - TUI client for Bluesky.
- [twitch-tui](https://github.com/Xithrius/twitch-tui) - Twitch chat in the terminal.
- [youtube-chat-rs](https://github.com/efekrskl/youtube-chat-rs) - A terminal UI for viewing YouTube live chat.

### 👨‍💻 System Administration

#### Monitoring, Diagnostics, and Logs

- [Aperture](https://github.com/stylebending/Aperture) - Diagnostic TUI for Windows power users.
- [bottom](https://github.com/ClementTsang/bottom) - Cross-platform graphical process/system monitor.
- [bpftop](https://github.com/jfernandez/bpftop) - A real-time monitor for Linux eBPF programs, showing runtime, event rate, and CPU usage.
- [cpustate-tui](https://github.com/mkulke/cpustate-tui) - A bootable TUI for inspecting x86_64 CPU state.
- [diskwatch](https://github.com/matthart1983/diskwatch) - Single-host, read-only disk diagnostics TUI.
- [erldash](https://github.com/sile/erldash) - A simple, terminal-based Erlang dashboard.
- [gentooplz](https://github.com/JustRoccat/gentooplz) - A live terminal dashboard for Gentoo package builds.
- [journalview](https://github.com/codervijo/journalview) - A TUI for browsing and filtering systemd journal logs.
- [kmon](https://github.com/orhun/kmon) - Linux Kernel Manager and Activity Monitor.
- [logss](https://github.com/todoesverso/logss) - A simple cli for logs splitting.
- [rrtop](https://github.com/wojciech-zurek/rrtop) - Redis monitoring (top like) app. rrtop -> \[r\]ust \[r\]edis \[top\].
- [tuistash](https://github.com/edmocosta/tuistash) - A TUI for monitoring Logstash.
- [vector](https://github.com/vectordotdev/vector) - A high-performance observability data pipeline.
- [winproc-tui](https://github.com/TX230/winproc-tui) - Process monitoring tool with live metrics, time-series graphs, A/B comparison.
- [zenith](https://github.com/bvaisvil/zenith) - Cross-platform monitoring tool for system stats.

#### Containers and Orchestration

- [b4n](https://github.com/fioletoven/b4n) - A terminal-based tool for browsing Kubernetes resources.
- [ducker](https://github.com/robertpsoane/ducker) - A terminal app for managing Docker containers, inspired by K9s.
- [kdash](https://github.com/kdash-rs/kdash) - A simple and fast dashboard for Kubernetes.
- [kftui](https://github.com/hcavarsan/kftray/blob/main/README.md#kftui) - A TUI to manage multiple kubectl port-forward commands, with support for UDP and Kubernetes proxy.
- [kubectl-watch](https://github.com/imuxin/kubectl-watch) - A kubectl plugin to provide a pretty delta change view of being watched Kubernetes resources.
- [kubetui](https://github.com/sarub0b0/kubetui) - TUI for real-time monitoring of Kubernetes resources.
- [oxker](https://github.com/mrjackwills/oxker) - Simple TUI to view & control Docker containers.
- [reeve](https://github.com/yetidevworks/reeve) - Manages a local web stack as per-user services: Caddy, Apache or nginx, per-vhost PHP-FPM versions, databases, local SSL, and wildcard DNS.
- [stevedore](https://github.com/takumiymd/stevedore) - A fast, keyboard-driven terminal UI for managing Docker containers and Compose stacks.

#### OS, Storage, and Package Management

- [brew-explorer](https://github.com/cosmincatalin/brew-explorer) - A TUI for exploring and managing your Homebrew packages with ease.
- [caligula](https://github.com/ifd3f/caligula) - A user-friendly, lightweight TUI for disk imaging.
- [dfdisk](https://github.com/tylerstyle/dfdisk) - Modern forensic disk imaging, damaged media rescue and evidence management.
- [dua-cli](https://github.com/Byron/dua-cli) - View disk space usage and delete unwanted data, fast.
- [lazyrsync](https://github.com/westpoint-io/lazyrsync) - A TUI for rsync: reusable profiles, a dry-run diff preview, and live run progress.
- [lemurs](https://github.com/coastalwhite/lemurs) - A customizable TUI login manager for Linux and BSD.
- [linutil](https://github.com/ChrisTitusTech/linutil) - A distro-agnostic toolbox designed to simplify everyday Linux tasks.
- [mirro-rs](https://github.com/rtkay123/mirro-rs) - An Arch Linux mirrorlist manager with a TUI.
- [pacman-utils](https://github.com/ankur3-101106/pacman-utils) - A keyboard-driven Arch Linux system manager with pacman, AUR, and reflector tools.
- [parui](https://github.com/Vonr/parui) - A TUI frontend for Arch User Repository helpers such as paru and yay.
- [systemctl-tui](https://github.com/rgwood/systemctl-tui) - A fast, simple TUI for interacting with systemd services and their logs.
- [systemd-manager-tui](https://github.com/matheus-git/systemd-manager-tui) - A program for managing systemd services through a TUI.
- [systeroid](https://github.com/orhun/systeroid) - A terminal UI for reading and changing Linux kernel parameters.

#### Batch, Database, and Cluster Operations

- [lazyslurm](https://github.com/hill/lazyslurm) - A lazygit-style terminal UI for Slurm. Monitor jobs, tail logs, and inspect nodes and partitions.
- [pgmon](https://github.com/nbari/pgmon) - A TUI for monitoring PostgreSQL databases.
- [pgtui](https://codeberg.org/kdwarn/pgtui) - A PostgreSQL TUI client that uses your terminal editor for inserts and updates.
- [slurmer](https://github.com/wjwei-handsome/Slurmer) - A TUI for monitoring and managing SLURM jobs.
- [sqwatch](https://github.com/fedonman/sqwatch) - A live SLURM queue dashboard.
- [Yozefu](https://github.com/MAIF/yozefu/) - A TUI for exploring data of a Kafka cluster.

### 🔌 Hardware and Embedded

- [blendr](https://github.com/dmtrKovalenko/blendr) - A terminal UI for browsing, connecting to, and inspecting Bluetooth Low Energy devices.
- [bluetui](https://github.com/pythops/bluetui) - A TUI for managing Bluetooth devices.
- [btlescan](https://github.com/ztroop/btlescan) - A terminal UI for scanning Bluetooth Low Energy devices and inspecting their services and characteristics.
- [ComChan](https://github.com/Vaishnav-Sabari-Girish/ComChan) - A minimal serial monitor with plotter TUI.
- [framework-tool-tui](https://github.com/grouzen/framework-tool-tui) - A TUI for controlling and monitoring Framework Computers hardware.
- [maccel](https://github.com/Gnarus-G/maccel) - A mouse acceleration driver for Linux, and a TUI to control some parameters.
- [macmon](https://github.com/vladkens/macmon) - Sudoless performance monitoring for Apple Silicon processors.
- [Mnyaoo32](https://github.com/intuis/mnyaoo32) - An IRC client for ESP32 devices with a Ratatui-based interface.
- [MTUI](https://github.com/inowattio/mtui) - A terminal client for Modbus industrial devices.
- [mxmon](https://github.com/yusufmo1/mxmon) - Sudoless Apple Silicon monitor with per-process watts, a live chassis heat map, and a JSON contract for scripts and agents.
- [napwatch](https://github.com/Tuguberk/napwatch) - Diagnoses and controls macOS power/battery behavior: dark wakes, Power Nap, live drain rate, and per-process power draw.
- [nightlight-tui](https://github.com/umutdinceryananer/nightlightd) - Dashboard for the nightlightd screen colour temperature daemon.
- [Phone-OS](https://github.com/Julien-cpsn/Phone-OS) - A modern Phone OS for ESP32 CYD (Cheap Yellow Display).
- [pumas](https://github.com/graelo/pumas) - Power Usage Monitor for Apple Silicon.
- [qmassa!](https://github.com/ulissesf/qmassa) - Displays GPU devices usage stats on Linux.
- [quokka](https://github.com/dutradotdev/quokka) - A TUI to inspect and tidy a USB-connected iPhone from macOS: storage, apps, media, syslog viewer.
- [suzui-rs](https://github.com/thatdevsherry/suzui-rs) - A terminal viewer for Suzuki engine data over the Suzuki Serial Data Line.
- [tegratop](https://github.com/pythops/tegratop) - TUI monitoring tool (top like) for Nvidia jetson boards.
- [thinkfan-tui](https://github.com/karjonas/thinkfan-tui) - A terminal-based Linux application for fan control and temperature monitoring on ThinkPad laptops.
- [v4l-tui](https://github.com/sermuns/v4l-tui) - Configure webcams on Linux via Video4Linux. TUI alternative to `v4l2-ctl`.

### 🔐 Security and Identity

- [chamber](https://github.com/mikeleppane/chamber) - A TUI for managing secrets.
- [cotp](https://github.com/replydev/cotp) - An encrypted command-line authenticator for time- and counter-based one-time passwords.
- [flawz](https://github.com/orhun/flawz) - A TUI for browsing software vulnerabilities from the Common Vulnerabilities and Exposures (CVE) database.
- [gpg-tui](https://github.com/orhun/gpg-tui) - A TUI for managing GnuPG encryption keys.
- [jwt-ui](https://github.com/jwt-rs/jwt-ui) - A command line UI for decoding/encoding JSON Web Tokens.
- [oak-keyring](https://github.com/OpenKeyring/oak-keyring) - A local-first password manager that keeps vault management interactive, keyboard-driven, and in the terminal.
- [passepartui](https://github.com/kardwen/passepartui) - A TUI for managing the pass password store.
- [YADB](https://github.com/izya4ka/yadb) - A TUI for discovering hidden directories and files on web servers.

### 📝 Productivity and Planning

#### Tasks, Projects, and Calendars

- [basilk](https://github.com/GabAlpha/basilk) - A TUI to manage your tasks with minimal kanban logic.
- [Jirust](https://github.com/Code-Militia/jirust) - A Jira TUI.
- [judo](https://github.com/giacomopiccinini/judo) - A multi-database TUI for ToDo lists.
- [kanban](https://github.com/kanban-rs/kanban) - TUI kanban board for projects management with sprint tracking and task prioritization.
- [lt](https://github.com/markmarkoh/lt) - An unofficial TUI client for Linear.app.
- [mirador](https://github.com/jchultarsky/mirador) - A personal dashboard with world clocks, calendar, weather, tasks, notes, a market watchlist and live CPU and network graphs.
- [Planner123](https://github.com/blackopsrepl/Planner123) - A local-first TUI calendar with an AI planning inbox that schedules your to-dos into reviewable proposals, plus Google Calendar sync and .ics import.
- [Rust-Kanban](https://github.com/yashs662/rust_kanban) - A kanban board for the terminal.
- [sc-cli](https://github.com/lnds/sc-cli) - A TUI for Shortcut (formerly know as Clubhouse) a project management tool for teams.
- [taskfinder](https://crates.io/crates/taskfinder) - Extract and display tasks from plain text files, hooking into your default terminal-based editor for editing.
- [taskwarrior-tui](https://github.com/kdheepak/taskwarrior-tui) - TUI for the Taskwarrior command-line task manager.
- [tatuin](https://github.com/panter-dsd/tatuin) - A terminal task manager that aggregates tasks from multiple providers.
- [td](https://github.com/holly-hacker/td) - A graph-based TUI to-do app.
- [tuxedo](https://github.com/webstonehq/tuxedo) - A fast, keyboard-driven terminal UI for todo.txt.
- [vault-tasks](https://github.com/louis-thevenet/vault-tasks) - TUI Markdown Task Manager.

#### Notes and Journaling

- [glues](https://github.com/gluesql/glues) - A sync-enabled TUI note-taking app with Git, CSV, and JSON support.
- [kimün](https://github.com/nico2sh/kimun) - A terminal-based Markdown note taking app that combines an interactive TUI with a scriptable CLI for automation.
- [MDDock](https://mddock.com) - Local-first Markdown notes app with a full ratatui TUI (live rendering, mermaid, grep) and an MCP server so AI agents read the same notes.
- [quick-note](https://github.com/daniel-valencia-ts/quick-note) - A simple note-taking tool.
- [revw](https://github.com/rlelf/revw) - A vim-like TUI for managing notes and resources.
- [rucola](https://github.com/Linus-Mussmaecher/rucola) - Terminal-based markdown note manager.
- [tmmpr](https://github.com/tanciaku/tmmpr) - Terminal mind mapper.
- [tui-journal](https://github.com/AmmarAbouZor/tui-journal) - Journaling/Notes-taking terminal-based app.

#### Finance and Markets

- [alphai-tui](https://github.com/makeev/alphai-tui) - A stock dashboard with quotes, candlestick charts, AI-scored news and SEC Form 4 insider activity.
- [budget-tracker-tui](https://github.com/Feromond/budget-tracker-tui) - A fast, keyboard-driven TUI for tracking expenses, tracking investments, and analyzing your budget with ease.
- [fitui](https://github.com/ayanchavand/fitui) - A terminal-based personal finance tracker and budgeting with TUI.
- [invoicepilot](https://github.com/adolfousier/invoicepilot) - A TUI for automating invoice and bank statement fetching from Gmail to Google Drive.
- [Rex](https://github.com/TheRustyPickle/Rex) - A TUI for managing Incomes and Expenses.
- [tickrs](https://github.com/tarkah/tickrs) - Stock market ticker in the terminal.

#### Focus, Habits, and Time

- [focusd](https://github.com/bibekbhusal0/focusd) - A terminal pomodoro timer with daemon, stats, history, streak, and nice interface.
- [isw](https://gitlab.com/thom-cameron/isw) - A simple terminal stopwatch application for pomodoro etc.
- [hustle-tracker](https://github.com/adolfousier/hustle-tracker) - A privacy-first TUI to track what apps you use and how long you spend on them.
- [Respire](https://github.com/ElevenJune/respire) - A breathing app to take a break directly from your terminal.
- [rusty-krab-manager](https://github.com/aryakaul/rusty-krab-manager) - A terminal time-management TUI inspired by the Pomodoro technique.
- [sprout](https://github.com/kb019/sprout) - A terminal habit tracker with GitHub-style activity heatmap, streak tracking and goal management.
- [timr-tui](https://github.com/sectore/timr-tui) - TUI to organize your time: Pomodoro, Countdown, Timer, Event.
- [void](https://github.com/p6laris/Void) - A focus app with built-in task management, streak tracking, and customizable break schedules.
- [work-tuimer](https://github.com/Kamyil/work-tuimer) - A TUI for easier time tracking each day, task-per-task with summaries.

### 📚 Reading and Learning

- [bbcli](https://github.com/hako/bbcli) - A terminal-based BBC News reader featuring a compact, numbered list interface with vim-like navigation.
- [eilmeldung](https://github.com/christo-auer/eilmeldung) - A TUI RSS reader based on the news_flash library inspired by Neovim and co.
- [exhaust](https://github.com/heyrict/exhaust) - A terminal app for doing exams.
- [fastcards](https://github.com/indium114/fastcards) - A CLI spaced-repetition flashcard study tool.
- [feedr](https://github.com/bahdotsh/feedr) - A terminal-based RSS/Atom feed reader with a TUI.
- [hg-tui](https://github.com/kaixinbaba/hg-tui) - A TUI for browsing and searching HelloGitHub's open-source project directory.
- [hncli](https://github.com/pierreyoda/hncli) - Hacker News read-only TUI.
- [kanash](https://github.com/benoitlx/kanash) - Learn Kana in your terminal.
- [leetrs](https://github.com/shadowmkj/leetrs) - A TUI for browsing, testing, and submitting LeetCode problems directly from your terminal.
- [ostt](https://github.com/kristoferlund/ostt) - Open Speech-to-Text recording tool with real-time volume metering and transcription.
- [rsstig](https://github.com/indium114/rsstig) - An unconventional terminal RSS/Atom reader.
- [scriptor](https://github.com/giacomopiccinini/scriptor) - A (medieval) local speech-to-text TUI & CLI.
- [synd](https://github.com/ymgyt/syndicationd) - A TUI feed viewer.
- [cargo-thesaurust](https://github.com/quietpigeon/cargo-thesaurust) - A terminal-based dictionary app.
- [tts-tui](https://github.com/lesleyrs/tts-tui) - Text to speech app that reads from clipboard.
- [ttyper](https://github.com/max-niederman/ttyper) - Terminal-based typing test.
- [ttypr](https://github.com/tanciaku/ttypr) - Terminal typing practice.

### 🎵 Music and Media

#### Music and Audio

- [angry-duiker](https://gitlab.com/fizzizist/angry-duiker-2) - A TUI for streaming music from network media servers using DLNA.
- [asak](https://github.com/chaosprint/asak) - A cross-platform audio recording/playback CLI tool.
- [audium](https://github.com/takashialpha/audium) - A keyboard-driven music app for people who live in the terminal.
- [bytebeat-rs](https://github.com/chaosprint/bytebeat-rs) - A terminal tool for generating music from bytebeat expressions.
- [chordflow](https://github.com/timvancann/chordflow) - A tool for practicing improvisation and mastering the guitar neck.
- [classfi](https://github.com/carmiac/classfi) - A focused streaming music player for classical music.
- [deezer-tui](https://github.com/Tatayoyoh/deezer-tui) - Deezer music TUI with included background player.
- [fum](https://github.com/qxb3/fum) - A fully ricable tui-based music client.
- [glicol-cli](https://github.com/glicol/glicol-cli) - Cross-platform music live coding in terminal.
- [lrxed](https://github.com/LunaPresent/lrxed) - A TUI application for synchronizing lyrics.
- [miditui](https://github.com/pinkpixel-dev/miditui) - A terminal MIDI player and multi-track piano roll visualizer with SoundFont synthesis.
- [myx](https://github.com/HaseebKhalid1507/Myx) - Modern Spotify player for the terminal. With reactive themes.
- [NoctaVox](https://github.com/Jaxx497/noctavox) - A lightweight, customizable TUI music player for local files.
- [oosc-rs](https://github.com/karasikq/oosc-rs) - A terminal synthesizer that builds sounds from layered waveforms.
- [O₂](https://github.com/coignard/o2) - Rust port of the ORCΛ esoteric programming language and terminal livecoding environment.
- [roon-tui](https://github.com/TheAppgineer/roon-tui) - Roon Remote for the terminal.
- [rs-pug](https://github.com/JustRoccat/rs-pug) - A Neovim-inspired asynchronous music player with a reactive audio visualizer.
- [rusty-pipes](https://github.com/dividebysandwich/rusty-pipes) - Sample-based, MIDI-controlled virtual pipe organ instrument.
- [scope-tui](https://github.com/alemidev/scope-tui) - A terminal audio scope with waveform, vector, and frequency views.
- [serenIT](https://github.com/ElevenJune/serenIT) - An ambient sound player directly from your terminal.
- [sgram-tui](https://github.com/arian-shamaei/sgram-tui) - A calibrated spectrogram analyzer for live mic or audio files, with labeled PNG figure export and a headless render mode.
- [smyx](https://github.com/ayanchavand/Smyx) - A sleek, beautiful music player for Navidrome / OpenSubsonic with dynamic themes.
- [sparkplayer](https://github.com/dividebysandwich/sparkplayer/tree/main) - A fun terminal based media player with album art and video support.
- [spotatui](https://github.com/LargeModGames/spotatui) - Spotify client with native streaming, synced lyrics, and audio visualization. A direct fork of spotify-tui with continued development and new features.
- [spotify-player](https://github.com/aome510/spotify-player) - An independently developed Spotify player with full feature parity.
- [spotify-tui](https://github.com/Rigellute/spotify-tui) - (Unmaintained) The original Spotify TUI; see spotatui (direct fork) or spotify-player (independent alternative).
- [trollstov](https://github.com/hikikones/trollstov) - A music player for the terminal where your files and their metadata are all you need.
- [Tuitar](https://github.com/orhun/tuitar) - A portable guitar training tool.
- [tusic](https://github.com/eminfedar/tusic) - Lightweight TUI Music Player. Play from YouTube or ~/Music folder easily.
- [Youta](https://github.com/vitaly-zdanevich/youta) - An audio player for YouTube, Yandex Music, podcasts, audiobooks, radio, and local files.
- [ytmusic-tui](https://github.com/WakaTaira/ytmusic-tui) - A TUI client for YouTube Music with Vim-style navigation and desktop media-control integration.
- [ytui-music](https://github.com/sudipghimire533/ytui-music) - Listen to music from YouTube in the terminal.

#### Books, Video, and Creative Media

- [Absotui](https://github.com/pdwaldrop/absotui) - A fast, keyboard-driven TUI client for Audiobookshelf (self-hosted audiobook/podcast server).
- [bookokrat](https://github.com/bugzmanov/bookokrat) - A full-featured EPUB / PDF e-book reader with Vim keybindings.
- [mal-cli](https://github.com/L4z3x/mal-cli) - A terminal client for MyAnimeList.
- [managarr](https://github.com/Dark-Alex-17/managarr) - A TUI and CLI for managing Sonarr, Radarr, and other Servarr applications.
- [manga-tui](https://github.com/josueBarretogit/manga-tui) - Terminal-based manga reader and downloader with image support.
- [MovieBox-TUI](https://github.com/mesamirh/MovieBox-Tui) - Terminal client for discovering and downloading movies and series, with playback in mpv, VLC, or IINA.
- [ratslate](https://github.com/azihsoyn/ratslate) - An infinite-canvas whiteboard driven by the mouse.
- [readio](https://github.com/hrhrng/readio) - A terminal ebook reader for EPUB, PDF, Markdown, and plain text with optional local text-to-speech.
- [red-table](https://github.com/volker-schukai/red-table) - A keyboard-driven terminal image browser for inspecting, comparing, and selecting photos.
- [TRNovel](https://github.com/yexiyue/TRNovel) - A terminal novel reader for local and network novels, with AI-assisted book-source generation and TTS playback.
- [tui-slides](https://github.com/Chleba/tui-slides) - Terminal presentation program with modern TUI.
- [visualvault](https://github.com/mikeleppane/visualvault) - A TUI for organizing media files.
- [ytsub](https://github.com/sarowish/ytsub) - A subscriptions only TUI YouTube client.

### 🎮 Games and Entertainment

- [a-puzzle-a-day](https://github.com/mrbjarksen/a-puzzle-a-day) - Generate and browse all solutions to A-Puzzle-A-Day.
- [astray](https://github.com/ekosachev/astray) - TUI-based space strategy game.
- [Battleship.rs](https://github.com/deepu105/battleship-rs) - Terminal-based Battleship game.
- [bigbrainwordle](https://github.com/kloki/bigbrainwordle) - A tool to help you cheat with the daily wordle.
- [Blackjack](https://github.com/skharchikov/blackjack) - Multiplayer Blackjack with a WebSocket server and a terminal client.
- [cgol-tui](https://github.com/jarjk/cgol-tui-rs) - Conway's Game of Life viewer with a TUI in Rust.
- [Chess-tui](https://github.com/thomas-mauran/chess-tui) - Terminal-based Chess game.
- [chessterm](https://github.com/ronaldsuwandi/chessterm) - A Rust-powered chess engine in a terminal.
- [confetty_rs](https://github.com/Handfish/confetty_rs) - Particle system (fireworks, stars) rendered in the terminal.
- [Connect-four](https://github.com/jesper-olsen/connect-four) - Connect-four; interactive game + perfect solver.
- [crosstui](https://github.com/matrixfrog/crossword) - Terminal-based crossword puzzle player.
- [cube timer](https://github.com/paarthmadan/cube) - A tui for cube timing, written in Rust.
- [Dealve](https://github.com/kurama/dealve-tui) - Browse game deals across Steam, GOG, Humble Bundle, Epic Games, and more from your terminal.
- [enimtui](https://codeberg.org/tranzystorekk/enimtui) - Terminal-based minesweeper knockoff.
- [flip7](https://github.com/ilyichv/flip7) - Terminal-based Flip7 game.
- [game-of-life-rs](https://github.com/kachark/game-of-life-rs) - Conway's Game of Life implemented in Rust and visualized with tui-rs.
- [gitlogue](https://github.com/unhappychoice/gitlogue) - A TUI screensaver that visualizes Git commit history in your terminal.
- [GitType](https://github.com/unhappychoice/gittype) - A CLI code-typing game that turns your source code into typing challenges.
- [Inertia](https://github.com/aclfe/inertia) - A 3D physics simulator in your terminal.
- [Maze TUI](https://github.com/agl-alexglopez/maze-tui) - Beautiful visualizations of common maze building and graph searching algorithms.
- [mlbt](https://github.com/mlb-rs/mlbt) - A tui for the MLB Statcast API. Watch a live game using Gameday, or check scores, standings, and stats.
- [modder-rs](https://github.com/JayanAXHF/modder-rs) - A practical TUI to manage and install mods for Minecraft.
- [oxycards](https://github.com/BrookJeynes/oxycards) - Quiz card application built within the terminal.
- [plastic](https://github.com/Amjad50/plastic) - NES emulator with extra ui implemented in ratatui.
- [poketex](https://github.com/ckaznable/poketex) - Simple Pokedex based on TUI.
- [private_poker](https://github.com/theOGognf/private_poker) - A poker library, server, client, and TUI.
- [Rebels in the sky](https://github.com/ricott1/rebels-in-the-sky) - P2P terminal game about spacepirates playing basketball across the galaxy.
- [rust-sadari-cli](https://github.com/24seconds/rust-sadari-cli) - Rust sadari game based on terminal! (Ghost leg or Amidakuji in another words).
- [sharad-ratatui](https://github.com/ProHaller/sharad_ratatui) - A text-based Shadowrun role-playing game.
- [Snake](https://github.com/kriskw1999/ratatui-snake) - Simple terminal based snake game.
- [ssHattrick](https://github.com/ricott1/sshattrick) - A multiplayer terminal version of the Hattrick football-management game, playable over SSH.
- [sued-rs](https://github.com/Danilo-Guedes/sued-rs) - A horror-themed recreation of SueD, the 2000s Brazilian prank oracle.
- [sxtetris](https://github.com/shixinhuang99/sxtetris) - A terminal Tetris game.
- [tage](https://github.com/jacopograndi/tage) - Turn based strategy game with multiplayer, empires and warfare.
- [termfarm](https://github.com/indium114/termfarm) - A simple idle farming game.
- [terminal.pong](https://github.com/IshmamR/terminal.pong) - Terminal based ping pong game.
- [tetris-tui](https://github.com/Axyl1410/tetris-tui) - Terminal Tetris following the Tetris Guideline.
- [Thardians](https://gitlab.com/thustle/thardians-rs) - Space Invaders for the terminal.
- [theattyr](https://github.com/orhun/theattyr) - A terminal theater for playing VT100 text art and animations.
- [tic-tac-toe](https://github.com/thomas-mauran/tic-tac-toe) - Terminal-based tic tac toe game.
- [ttysvr](https://github.com/cxreiff/ttysvr) - Screen saver for your terminal.
- [WOPR TUI 2026](https://github.com/ankurCES/WOPR_TUI_2026) - A WarGames-inspired Cold War simulation TUI with AI-powered scenarios, DEFCON escalation, and multi-language intelligence intercepts.
- [wordl](https://github.com/palerdot/wordl-rs) - Terminal-based Wordle game. Web like experience with keyboard hints and guess reveal animations.

### 🔬 Science, Math, and Exploration

- [fractouille](https://github.com/PottierLoic/Fractouille) - A simple fractal explorer running in your terminal.
- [lpl](https://github.com/SOF3/lpl) - Command-line plotting for real-time CSV and JSON streams.
- [meteo-tui](https://github.com/16arpi/meteo-tui) - French weather app in the command line.
- [numr](https://github.com/nasedkinpv/numr) - A natural language calculator with unit/currency conversions and vim-style keybindings.
- [oeis-tui](https://github.com/hako/oeis-tui) - A TUI and CLI for browsing the On-Line Encyclopedia of Integer Sequences (OEIS) in the terminal.
- [Raijin](https://github.com/MasonStooksbury/Raijin) - A free, simple weather TUI that pulls data without the need for an API key, account, or subscription.
- [rsfrac](https://github.com/SkwalExe/rsfrac) - Terminal based fractal explorer, including Mandelbrot, Burning Ship, and Julia.
- [seqsizzle](https://github.com/ChangqingW/SeqSizzle) - A terminal pager for viewing and searching FASTA and FASTQ DNA sequence files.
- [SeqTUI](https://github.com/ranwez-search/SeqTUI) - A terminal-based viewer and command-line toolkit for molecular sequences.
- [tenki](https://github.com/ckaznable/tenki) - A tty-clock with weather effect.
- [termCA](https://github.com/fabiooo4/termCA) - Interactive TUI Cellular Automata simulator.
- [tgv](https://github.com/zeqianli/tgv) - A terminal genome browser with Vim-style navigation.
- [tracker](https://github.com/ShenMian/tracker) - A terminal-based real-time satellite tracking and orbit prediction application.

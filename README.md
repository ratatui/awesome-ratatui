<!--lint disable awesome-git-repo-age-->

# Awesome Ratatui [![Awesome](https://awesome.re/badge-flat2.svg)](https://awesome.re)

[<img src="https://github.com/ratatui.png" align="right" width="100">](https://ratatui.rs)

Here you will find a list of TUI crates and applications that are made for or using [`ratatui`](https://crates.io/crates/ratatui) and [`tui`](https://crates.io/crates/tui).

<!--lint disable awesome-toc-->

## Contents

- [📦 Libraries](#-libraries)
  - [🏗️ Frameworks](#%EF%B8%8F-frameworks)
  - [🧩 Widgets](#-widgets)
  - [🔧 Utilities](#-utilities)
- [💻 Apps](#-apps)
  - [⌨️ Development Tools](#%EF%B8%8F-development-tools)
  - [🕹️ Games and Entertainment](#%EF%B8%8F-games-and-entertainment)
  - [🚀 Productivity and Utilities](#-productivity-and-utilities)
  - [🎼 Music and Media](#-music-and-media)
  - [🌐 Networking and Internet](#-networking-and-internet)
  - [👨‍💻 System Administration](#-system-administration)
  - [🌌 Other](#-other)

Aside from those listed here, many other apps and libraries can be easily be found via the reverse dependencies on crates.io and GitHub:

- <https://crates.io/crates/ratatui/reverse_dependencies>
- <https://crates.io/crates/tui/reverse_dependencies>
- <https://github.com/ratatui/ratatui/network/dependents>
- <https://github.com/fdehau/tui-rs/network/dependents?package_id=UGFja2FnZS0zMjE3MzkzMDMx>

## 📦 Libraries

### 🏗️ Frameworks

- [bevy_ratatui_camera](https://github.com/cxreiff/bevy_ratatui_camera) - A bevy plugin for rendering your bevy app to the terminal using ratatui.
- [egui-ratatui](https://github.com/gold-silver-copper/egui_ratatui) - A ratatui backend that is also an egui widget. Deploy on web with WASM or ship natively with bevy, macroquad, or eframe.
- [mousefood](https://github.com/j-g00da/mousefood) - An embedded-graphics backend for Ratatui.
- [ratatui-uefi](https://github.com/reubeno/tui-uefi) - A ratatui backend for use in UEFI environments.
- [ratzilla](https://github.com/orhun/ratzilla) - Build terminal-themed web applications with Ratatui and WebAssembly.
- [rlt](https://crates.io/crates/rlt) - A universal load testing framework for Rust, with real-time tui support.
- [soft_ratatui](https://github.com/gold-silver-copper/soft_ratatui) - A software rendering backend for ratatui. No GPU required. TUI everywhere.
- [tui-react](https://crates.io/crates/tui-react) - TUI widgets using a react-like paradigm.
- [tui-realm](https://crates.io/crates/tuirealm) - A ratatui framework inspired by Elm and React.
- [webatui](https://github.com/TylerBloom/webatui) - An integration between the Yew and Ratatui crates for making TUI-themed WASM webapps.
- [widgetui](https://crates.io/crates/widgetui) - A bevy-like widget system for ratatui and crossterm.

### 🧩 Widgets

- [edtui](https://github.com/preiter93/edtui) - A TUI based vim-inspired editor widget for ratatui.
- [ratatui-explorer](https://github.com/tatounee/ratatui-explorer) - A simple library for creating file explorer for ratatui.
- [ratatui-image](https://crates.io/crates/ratatui-image) - An image widget for ratatui, supporting sixels and unicode-halfblocks.
- [ratatui-splash-screen](https://github.com/orhun/ratatui-splash-screen) - A widget to turn any image to a splash screen.
- [ratatui-textarea](https://crates.io/crates/ratatui-textarea) - A simple yet powerful editor widget for ratatui. Fork of `tui-textarea`.
- [term-rustdoc](https://github.com/zjp-CN/term-rustdoc) - A TUI for Rust docs that aims to improve the UX on tree view and generic code.
- [throbber-widgets-tui](https://crates.io/crates/throbber-widgets-tui) - A widget that displays throbber.
- [tui-additions](https://crates.io/crates/tui-additions) - Additions to the rust tui crate.
- [tui-big-text](https://crates.io/crates/tui-big-text) - A simple ratatui widget for displaying big text using the `font8x8` crate.
- [tui-logger](https://crates.io/crates/tui-logger) - Logger with smart widget for ratatui.
- [tui-menu](https://github.com/shuoli84/tui-menu) - A menu widget for ratatui ecosystem.
- [tui-nodes](https://crates.io/crates/tui-nodes) - Node graph visualization.
- [tui-popup](https://github.com/joshka/tui-popup) - A Popup widget for Ratatui.
- [tui-prompts](https://crates.io/crates/tui-prompts) - A library for building interactive prompts for ratatui.
- [tui-rain](https://github.com/levilutz/tui-rain) - A widget to generate various rain effects.
- [tui-scrollview](https://crates.io/crates/tui-scrollview) - A container that provides a scrolling view at a larger area.
- [tui-term](https://crates.io/crates/tui-term) - A pseudoterminal widget for ratatui.
- [tui-textarea](https://crates.io/crates/tui-textarea) - A simple yet powerful text editor widget for ratatui and tui-rs.
- [tui-tree-widget](https://crates.io/crates/tui-tree-widget) - Tree widget for ratatui.
- [tui-widget-list](https://crates.io/crates/tui-widget-list) - A versatile list implementation for ratatui.

### 🔧 Utilities

- [ansi-to-tui](https://crates.io/crates/ansi-to-tui) - A library to convert ansi color coded text into `ratatui::text::Text`.
- [bevy_ratatui](https://github.com/joshka/bevy_ratatui) - A rust crate to use Ratatui in a Bevy App.
- [color-to-tui](https://crates.io/crates/color-to-tui) - Parse colors and convert them to `ratatui::style::Colors`.
- [coolor](https://github.com/Canop/coolor) - Tiny color conversion library for TUI application builders.
- [ratatui-macros](https://github.com/kdheepak/ratatui-macros) - Macros for simplifying boilerplate for creating UI using Ratatui.
- [tachyonfx](https://github.com/junkdog/tachyonfx) - A shader-like effects library for ratatui.
- [tui-input](https://crates.io/crates/tui-input) - A headless input library for TUI apps.

## 💻 Apps

### ⌨️ Development Tools

- [ATAC](https://github.com/Julien-cpsn/ATAC) - A feature-full TUI API client for your terminal.
- [BugStalker](https://github.com/godzie44/BugStalker) - Modern rust debugger for Linux x86-64.
- [burn](https://github.com/burn-rs/burn) - Comprehensive Deep Learning framework in Rust.
- [cargo-selector](https://github.com/lusingander/cargo-selector) - Cargo subcommand to select and execute binary/example targets.
- [desed](https://github.com/SoptikHa2/desed) - Debugging tool for sed scripts.
- [gimoji](https://github.com/zeenix/gimoji) - Makes it easy to add emojis to your Git commit messages.
- [gitu](https://github.com/altsem/gitu) - A TUI Git client inspired by Magit.
- [gitui](https://github.com/extrawurst/gitui) - Terminal UI for Git.
- [gobang](https://github.com/TaKO8Ki/gobang) - Cross-platform TUI database management tool.
- [joshuto](https://github.com/kamiyaa/joshuto) - Ranger-like terminal file manager written in Rust.
- [lazyjj](https://github.com/Cretezy/lazyjj) - TUI for the Jujutsu/jj VCS.
- [Maelstrom](https://github.com/maelstrom-software/maelstrom) - Maelstrom is a fast test runner that runs every test in its own container locally or distributed.
- [material](https://github.com/azorng/material) - A material design color palette for the terminal.
- [nomad](https://github.com/JosephLai241/nomad) - Customizable next-gen tree command with Git integration and TUI.
- [Oatmeal](https://github.com/dustinblackman/oatmeal) - Terminal UI to chat with large language models (LLM) using different model backends, and integrations with your favourite editors!
- [openapi-tui](https://github.com/zaghaghi/openapi-tui) - Terminal UI to list, browse and run APIs defined with openapi spec.
- [rainfrog](https://github.com/achristmascarl/rainfrog) - A database management TUI for Postgres.
- [repgrep](https://github.com/acheronfail/repgrep) - An interactive replacer for ripgrep that makes it easy to find and replace across files on the command line.
- [scooter](https://github.com/thomasschafer/scooter) - Interactive find and replace in the terminal.
- [serie](https://github.com/lusingander/serie) - A rich git commit graph in your terminal.
- [Serpl](https://github.com/yassinebridi/serpl) - A simple terminal UI for search and replace, ala VS Code.
- [slumber](https://github.com/LucasPickering/slumber) - Terminal-based HTTP/REST client.
- [TaskUI](https://github.com/thmshmm/taskui) - Simple Terminal UI for Task / taskfile.dev.
- [tenere](https://github.com/pythops/tenere) - TUI interface for LLMs written in Rust.
- [tongo](https://github.com/drewzemke/tongo) - A TUI for MongoDB. 
- [tracexec](https://github.com/kxxt/tracexec) - Tracer for execve{,at} and pre-exec behavior, launcher for debuggers.
- [Yazi](https://github.com/sxyazi/yazi) - Blazing fast terminal file manager written in Rust, based on async I/O.

### 🕹️ Games and Entertainment

- [a-puzzle-a-day](https://github.com/mrbjarksen/a-puzzle-a-day) - Generate and browse all solutions to A-Puzzle-A-Day.
- [astray](https://github.com/Vinermy/astray) - TUI-based space strategy game.
- [Battleship.rs](https://github.com/deepu105/battleship-rs) - Terminal-based Battleship game.
- [bigbrainwordle](https://github.com/kloki/bigbrainwordle) - A tool to help you cheat with the daily wordle.
- [chessterm](https://github.com/ronaldsuwandi/chessterm) - A Rust-powered chess engine in a terminal.
- [Chess-tui](https://github.com/thomas-mauran/chess-tui) - Terminal-based Chess game.
- [enimtui](https://codeberg.org/tranzystorekk/enimtui) - Terminal-based minesweeper knockoff.
- [cgol-tui](https://github.com/jeromeschmied/cgol-tui-rs) - Conway's Game of Life viewer with a TUI in Rust.
- [game-of-life-rs](https://github.com/kachark/game-of-life-rs) - Conway's Game of Life implemented in Rust and visualized with tui-rs.
- [Maze TUI](https://github.com/agl-alexglopez/maze-tui) - Beautiful visualizations of common maze building and graph searching algorithms.
- [minesweep](https://github.com/cpcloud/minesweep-rs) - Terminal-based Minesweeper game.
- [oxycards](https://github.com/BrookJeynes/oxycards) - Quiz card application built within the terminal.
- [plastic](https://github.com/Amjad50/plastic) - NES emulator with extra ui implemented in ratatui.
- [private_poker](https://github.com/theOGognf/private_poker) - A poker library, server, client, and TUI.
- [Rebels in the sky](https://github.com/ricott1/rebels-in-the-sky) - P2P terminal game about spacepirates playing basketball across the galaxy.
- [rust-sadari-cli](https://github.com/24seconds/rust-sadari-cli) - Rust sadari game based on terminal! (Ghost leg or Amidakuji in another words).
- [sharad-ratatui](https://github.com/ProHaller/sharad_ratatui) - A text-based Shadowrun role-playing game.
- [Snake](https://github.com/kriskw1999/ratatui-snake) - Simple terminal based snake game.
- [ssHattrick](https://github.com/ricott1/sshattrick) - Play Hattrick in your terminal over SSH.
- [sxtetris](https://github.com/shixinhuang99/sxtetris) - A terminal Tetris game.
- [tage](https://github.com/jacopograndi/tage) - Turn based strategy game with multiplayer, empires and warfare.
- [Thardians](https://gitlab.com/thustle/thardians-rs) - Space Invaders for the terminal.
- [tic-tac-toe](https://github.com/thomas-mauran/tic-tac-toe) - Terminal-based tic tac toe game.
- [wordl](https://github.com/palerdot/wordl-rs) - Terminal-based Wordle game. Web like experience with keyboard hints and guess reveal animations.

### 🎼 Music and Media

- [angry-duiker](https://gitlab.com/fizzizist/angry-duiker-2) - A TUI for streaming music from a DLNA server.
- [bytebeat-rs](https://github.com/chaosprint/bytebeat-rs) - A TUI for bytebeat.
- [chordflow](https://github.com/timvancann/chordflow) - A tool for practicing improvisation and mastering the guitar neck.
- [fum](https://github.com/qxb3/fum) - A fully ricable tui-based music client.
- [glicol-cli](https://github.com/glicol/glicol-cli) - Cross-platform music live coding in terminal.
- [managarr](https://github.com/Dark-Alex-17/managarr) - A TUI and CLI for managing all your Servarrs.
- [manga-tui](https://github.com/josueBarretogit/manga-tui) - Terminal-based manga reader and downloader with image support.
- [oosc-rs](https://github.com/karasikq/oosc-rs) - An additive wavetable synthesizer for terminal.
- [roon-tui](https://github.com/TheAppgineer/roon-tui) - Roon Remote for the terminal.
- [scope-tui](https://github.com/alemidev/scope-tui) - A simple oscilloscope/vectorscope/spectroscope for your terminal.
- [spotify-player](https://github.com/aome510/spotify-player) - A Spotify player in the terminal with full feature parity.
- [spotify-tui](https://github.com/Rigellute/spotify-tui) - (DEPRECATED; See `spotify-player`) Spotify client for the terminal.
- [twitch-tui](https://github.com/Xithrius/twitch-tui) - Twitch chat in the terminal.
- [ytui-music](https://github.com/sudipghimire533/ytui-music) - Listen to music from YouTube in the terminal.

### 🌐 Networking and Internet

- [AdGuardian-Term](https://github.com/Lissy93/AdGuardian-Term) - Real-time traffic monitoring and statistics for AdGuard Home.
- [adsb_deku/radar](https://github.com/wcampbell0x2a/adsb_deku#radar-tui) - TUI for displaying ADS-B data from aircraft.
- [bandwhich](https://github.com/imsnif/bandwhich) - Displays network utilization by process.
- [conclusive](https://github.com/mrusme/conclusive) - A command line client for Plausible Analytics.
- [CuTE](https://github.com/PThorpe92/CuTE) - A libcurl powered HTTP Client with API-key/request mgmt and vim keybindings.
- [discovery-rs](https://github.com/JustPretender/discovery-rs) - An utility to discover mDNS services on your network.
- [gping](https://github.com/orf/gping/) - Ping tool with a graph.
- [impala](https://github.com/pythops/impala) - TUI for managing wifi on Linux.
- [mqttui](https://github.com/EdJoPaTo/mqttui) - MQTT client for subscribing or publishing to topics.
- [netscanner](https://github.com/Chleba/netscanner) - Network scanning tool.
- [nordvpn-tui](https://github.com/Degra02/nordvpn-tui) - A TUI for NordVPN.
- [oha](https://github.com/hatoo/oha) - Top-like monitoring tool for HTTP(S) traffic.
- [oryx](https://github.com/pythops/oryx) - A TUI for sniffing network traffic using eBPF.
- [rrtop](https://github.com/wojciech-zurek/rrtop) - Redis monitoring (top like) app. rrtop -> \[r\]ust \[r\]edis \[top\].
- [rustmission](https://codeberg.org/micielski/rustmission) - TUI for the Transmission daemon.
- [sensor-vision](https://github.com/jcfromsiberia/sensor-vision) - TUI Client for TeamViewer IoT MQTT API for managing IoT Sensors and Metrics.
- [terminusdm](https://github.com/sumoduduk/terminusdm) - Cross Platform Terminal Download Manager.
- [termscp](https://github.com/veeso/termscp) - A feature rich terminal UI file transfer and explorer with support for SCP/SFTP/FTP/S3/SMB.
- [trippy](https://github.com/fujiapple852/trippy) - Network diagnostic tool.
- [tsuchita](https://github.com/kamiyaa/tsuchita) - Client-server notification center for dbus desktop notifications.
- [vector](https://github.com/vectordotdev/vector) - A high-performance observability data pipeline.
- [vincenzo](https://github.com/gabrieldemian/vincenzo) - A bittorrent client for the terminal with vim-like keybindings.
- [wireman](https://github.com/preiter93/wireman) - A gRPC client for the terminal.

### 🚀 Productivity and Utilities

- [atuin](https://github.com/atuinsh/atuin) - Magical shell history.
- [basilk](https://github.com/GabAlpha/basilk) - A TUI to manage your tasks with minimal kanban logic.
- [binsider](https://github.com/orhun/binsider) - A TUI for analyzing binary files.
- [blendr](https://github.com/dmtrKovalenko/blendr) - The hacker's BLE (bluetooth low energy) browser terminal app.
- [bluetui](https://github.com/pythops/bluetui) - A TUI for managing Bluetooth devices.
- [btlescan](https://github.com/ztroop/btlescan) - Bluetooth Low Energy (BTLE) scanner and GATT viewer.
- [csvlens](https://github.com/YS-L/csvlens) - Command line csv viewer.
- [dead-ringer](https://github.com/ztroop/dead-ringer) - Binary diff tool for Hex/ASCII analysis.
- [diskonaut](https://github.com/imsnif/diskonaut) - Terminal-based disk space navigator.
- [exabind](https://github.com/junkdog/exabind) - An animated TUI for viewing KDE shortcuts.
- [exhaust](https://github.com/heyrict/exhaust) - Exhaust all your possibilities for the next coming exam.
- [flawz](https://github.com/orhun/flawz) - A TUI for browsing security vulnerabilities (CVEs).
- [fzf-make](https://github.com/kyu08/fzf-make) - A command line tool that executes make target using fuzzy finder with preview window.
- [get_blessed_rs](https://github.com/josueBarretogit/get_blessed_rs) - Get the best crates for your rust projects, curated by blessed.rs.
- [glues](https://github.com/gluesql/glues) - A sync-enabled TUI note-taking app with Git, CSV, and JSON support.
- [gpg-tui](https://github.com/orhun/gpg-tui) - Manage your GnuPG keys with ease!.
- [igrep](https://github.com/konradsz/igrep) - Interactive Grep.
- [isw](https://gitlab.com/thom-cameron/isw) - A simple terminal stopwatch application for pomodoro etc.
- [Jirust](https://github.com/moali87/jirust) - A Jira TUI.
- [jwt-ui](https://github.com/jwt-rs/jwt-ui) - A command line UI for decoding/encoding JSON Web Tokens.
- [kbt](https://github.com/bloznelis/kbt) - Keyboard tester in terminal.
- [kraban](https://github.com/TadoTheMiner/kraban) - Rust kanban board tui inspired by basilk.
- [lazy-etherscan](https://github.com/woxjro/lazy-etherscan) - A Simple Terminal UI for the Ethereum Blockchain Explorer.
- [linutil](https://github.com/ChrisTitusTech/linutil) - A distro-agnostic toolbox designed to simplify everyday Linux tasks.
- [md-tui](https://github.com/henriklovhaug/md-tui) - Markdown renderer in the terminal.
- [meteo-tui](https://github.com/16arpi/meteo-tui) - French weather app in the command line.
- [mprocs](https://github.com/pvolok/mprocs) - Run multiple commands in parallel and shows output of each command separately.
- [otree](https://github.com/fioncat/otree) - A command line tool to view objects (JSON/YAML/TOML) in TUI tree widget.
- [passepartui](https://github.com/kardwen/passepartui) - A TUI for pass.
- [regect](https://github.com/kloki/regect) - A regex101 like tool for the cli.
- [Rex](https://github.com/TheRustyPickle/Rex) - A TUI for managing Incomes and Expenses.
- [rucola](https://github.com/Linus-Mussmaecher/rucola) - Terminal-based markdown note manager.
- [Rust-Kanban](https://github.com/yashs662/rust_kanban) - A kanban board for the terminal.
- [rusty-krab-manager](https://github.com/aryakaul/rusty-krab-manager) - Rime management TUI in Rust.
- [stu](https://github.com/lusingander/stu) - A TUI for AWS S3.
- [synd](https://github.com/ymgyt/syndicationd) - A TUI feed viewer.
- [sheetsui](https://github.com/zaphar/sheetsui) - A terminal based spreadsheet application.
- [tabiew](https://github.com/shshemi/tabiew) - A lightweight TUI app to view and query CSV files.
- [taskwarrior-tui](https://github.com/kdheepak/taskwarrior-tui) - TUI for the Taskwarrior command-line task manager.
- [td](https://github.com/holly-hacker/td) - A graph-based TUI to-do app.
- [television](https://github.com/alexpasmantier/television) - A blazingly fast general purpose fuzzy finder for your terminal.
- [thesaurust](https://github.com/QuietPigeon2001/thesaurust) - A terminal-based dictionary app.
- [tickrs](https://github.com/tarkah/tickrs) - Stock market ticker in the terminal.
- [tim:r](https://github.com/sectore/timr) - A TUI for organizing your time: Pomodoro, Countdown, Timer.
- [todolist-rust](https://github.com/ebubekirgungor/todolist-rust) - A terminal-based simple to-do app.
- [traceview](https://github.com/javaLux/traceview) - Tracing and viewing your files and resource landscape.
- [tts-tui](https://github.com/lesleyrs/tts-tui) - Text to speech app that reads from clipboard.
- [ttyper](https://github.com/max-niederman/ttyper) - Terminal-based typing test.
- [tui-journal](https://github.com/AmmarAbouZor/tui-journal) - Journaling/Notes-taking terminal-based app.
- [tui-slides](https://github.com/Chleba/tui-slides) - Terminal presentation program with modern TUI.
- [tuistash](https://github.com/edmocosta/tuistash) - A TUI for monitoring Logstash.
- [vault-tasks](https://github.com/louis-thevenet/vault-tasks) - TUI Markdown Task Manager.
- [wiper](https://github.com/ikebastuz/wiper) - Disk space analyzer and cleanup tool.

### 🤡 Social Media

- [Chat-gRPC](https://github.com/Atheer2104/chat-grpc) - A Real-time Chat Microservice built in Rust using gRPC, including a TUI client.
- [iamb](https://github.com/ulyssa/iamb) - A matrix chat client with vim keybindings.
- [lobtui](https://github.com/pythops/lobtui) - TUI for lobste.rs website.
- [nostui](https://github.com/akiomik/nostui) - A TUI client for Nostr.
- [termchat](https://github.com/lemunozm/termchat) - Terminal chat through the LAN with video streaming and file transfer.
- [tgt](https://github.com/FedericoBruzzone/tgt) - A TUI for Telegram written in Rust.
- [tuisky](https://github.com/sugyan/tuisky) - TUI client for Bluesky.

### 👨‍💻 System Administration

- [bottom](https://github.com/ClementTsang/bottom) - Cross-platform graphical process/system monitor.
- [bpftop](https://github.com/Netflix/bpftop) - Dynamic real-time view of running eBPF programs.
- [caligula](https://github.com/ifd3f/caligula) - A user-friendly, lightweight TUI for disk imaging.
- [dua-cli](https://github.com/Byron/dua-cli) - View disk space usage and delete unwanted data, fast.
- [ducker](https://github.com/robertpsoane/ducker) - A terminal app for managing docker containers, inspired by K9s.
- [erldash](https://github.com/sile/erldash) - A simple, terminal-based Erlang dashboard.
- [kdash](https://github.com/kdash-rs/kdash) - A simple and fast dashboard for Kubernetes.
- [kftui](https://github.com/hcavarsan/kftray/blob/main/README.md#kftui) - A TUI to manage multiple kubectl port-forward commands, with support for UDP and K8s proxy.
- [kmon](https://github.com/orhun/kmon) - Linux Kernel Manager and Activity Monitor.
- [kubectl-watch](https://github.com/imuxin/kubectl-watch) - A kubectl plugin to provide a pretty delta change view of being watched kubernetes resources.
- [kubetui](https://github.com/sarub0b0/kubetui) - TUI for real-time monitoring of Kubernetes resources.
- [logss](https://github.com/todoesverso/logss) - A simple cli for logs splitting.
- [macmon](https://github.com/vladkens/macmon) - Sudoless performance monitoring for Apple Silicon processors.
- [mirro-rs](https://github.com/rtkay123/mirro-rs) - An Arch Linux mirrorlist manager with a TUI.
- [oxker](https://github.com/mrjackwills/oxker) - Simple TUI to view & control docker containers.
- [parui](https://github.com/Vonr/parui) - Simple TUI frontend for paru or yay.
- [pumas](https://github.com/graelo/pumas) - Power Usage Monitor for Apple Silicon.
- [qmassa!](https://github.com/ulissesf/qmassa) - Displays GPU devices usage stats on Linux.
- [systemctl-tui](https://github.com/rgwood/systemctl-tui) - A fast, simple TUI for interacting with systemd services and their logs.
- [systeroid](https://github.com/orhun/systeroid) - A more powerful alternative to sysctl(8) with a terminal user interface.
- [tegratop](https://github.com/pythops/tegratop) - TUI monitoring tool (top like) for Nvidia jetson boards.
- [xplr](https://github.com/sayanarijit/xplr) - Hackable, minimal, and fast TUI file explorer.
- [ytop](https://github.com/cjbassi/ytop) - TUI system monitor for Linux.
- [zenith](https://github.com/bvaisvil/zenith) - Cross-platform monitoring tool for system stats.
- [journalview](https://github.com/codervijo/journalview) - Journalctl log viewer.

### 🌌 Other

- [confetty_rs](https://github.com/Handfish/confetty_rs) - Particle system (fireworks, stars) rendered in the terminal.
- [cotp](https://github.com/replydev/cotp) - Command-line TOTP/HOTP authenticator app.
- [cube timer](https://github.com/paarthmadan/cube) - A tui for cube timing, written in Rust.
- [hg-tui](https://github.com/kaixinbaba/hg-tui) - TUI for viewing the hellogithub.com website.
- [hncli](https://github.com/pierreyoda/hncli) - Hacker News read-only TUI.
- [hwatch](https://github.com/blacknon/hwatch) - Alternative watch command with command history and diffs.
- [kanash](https://github.com/benoitlx/kanash) - Learn Kana in your terminal.
- [lemurs](https://github.com/coastalwhite/lemurs) - A customizable TUI login manager for Linux and BSD.
- [lpl](https://github.com/SOF3/lpl) - Command-line plotting for real-time CSV and JSON streams.
- [maccel](https://github.com/Gnarus-G/maccel) - A mouse acceleration driver for linux, and a TUI to control some parameters.
- [nyaa](https://github.com/Beastwick18/nyaa) - A nyaa.si tui tool for browsing and downloading torrents.
- [poketex](https://github.com/ckaznable/poketex) - Simple Pokedex based on TUI.
- [seqsizzle](https://github.com/ChangqingW/SeqSizzle) - A pager for viewing FASTQ files with fuzzy matching and coloring.
- [tenki](https://github.com/ckaznable/tenki) - A tty-clock with weather effect.
- [tgv](https://github.com/zeqianli/tgv) - Explore human genomes in the terminal.
- [theattyr](https://github.com/orhun/theattyr) - A terminal theater for playing VT100 art and animations.
- [tracker](https://github.com/ShenMian/tracker) - A terminal-based real-time satellite tracking and orbit prediction application.
- [ttysvr](https://github.com/cxreiff/ttysvr) - Screen saver for your terminal.

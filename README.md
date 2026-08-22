<div align="center">

# wordle-tui

**The daily Wordle, in your terminal. Nothing to install — just SSH in.**

```
ssh wordle.quacklabs.io
```

[![CI](https://github.com/GameboyColor32/wordle-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/GameboyColor32/wordle-tui/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

<img src="docs/demo.gif" alt="wordle-tui gameplay" width="600">

</div>

---

## What is this?

A faithful implementation of Wordle as a terminal UI, written in Rust with
[ratatui](https://ratatui.rs). It plays the **real word of the day** — the same
puzzle everyone else is solving — and it's hosted over SSH, so anyone with a
terminal can play without cloning, compiling, or installing anything.

Your stats and streak follow you between sessions, identified by your SSH public
key. No accounts, no passwords, no signup screen.

## Play

```bash
# Play the hosted daily puzzle
ssh wordle.quacklabs.io

# ...or run it locally
cargo install --git https://github.com/GameboyColor32/wordle-tui wordle-play
wordle-play
```

| Key | Action |
| --- | --- |
| `a`–`z` | Type a letter |
| `Enter` | Submit guess |
| `Backspace` | Delete a letter |
| `Tab` | Toggle stats panel |
| `?` | Help |
| `Ctrl-C` / `q` | Quit |

## Features

- **The real daily word**, pulled from the public NYT puzzle endpoint and cached
  server-side (one fetch per day, shared by every player).
- **Zero-install play over SSH** — a full ratatui interface rendered into the SSH
  channel, resize-aware and 256-color/truecolor capable.
- **Persistent stats without accounts** — streaks, win distribution, and guess
  history keyed to your public key fingerprint.
- **Offline-safe** — if the upstream endpoint is unreachable, the server falls
  back to a deterministic local word list rather than going down.
- **Shareable results** — the familiar 🟩🟨⬛ grid, copied to your clipboard via
  OSC 52 (works through SSH).
- **Hard mode**, because otherwise what's the point.

## How it works

The interesting problem here isn't Wordle — it's serving a TUI to strangers over
SSH without writing the game twice.

```
                    ┌──────────────────────────────────────┐
   ssh client ─────▶│  wordle-ssh                          │
                    │  russh server · one session per conn │
                    │  pty bytes ──▶ Event                 │
                    │  Frame ──▶ ANSI ──▶ channel          │
                    └───────────────┬──────────────────────┘
                                    │  Event / draw(Frame)
   local terminal ──▶ wordle-play ──┤
                                    ▼
                    ┌──────────────────────────────────────┐
                    │  wordle-app   ratatui UI + state      │
                    │  update(Event) -> Effect · draw()     │
                    └───────────────┬──────────────────────┘
                                    │  Word in, Feedback out
                                    ▼
                    ┌──────────────────────────────────────┐
                    │  wordle-core  rules engine, no I/O    │
                    │  ◀── Arc<Puzzle> ── wordle-daily      │
                    └──────────────────────────────────────┘
```

`wordle-app` never touches stdin, stdout, or a socket. It consumes an `Event`
enum and renders into a ratatui `Frame`. That single boundary is what lets the
same UI run in your local terminal and over an SSH pty, and what makes the whole
thing testable with `TestBackend` snapshots instead of a human squinting at a
screen.

`wordle-core` has no dependencies, no async, and no knowledge that a terminal
exists. The secret word is wrapped in a redacting `Secret` newtype that is
unreachable through the public API until the game is over — so no render path,
log line, or panic message can leak today's answer. See
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the details.

## Project layout

```
crates/
  wordle-core/    Rules engine. Pure, dependency-free, heavily tested.
  wordle-daily/   Word-of-the-day client, disk cache, offline fallback.
  wordle-app/     ratatui UI and app state machine. Transport-agnostic.
  wordle-ssh/     russh server. Custom Backend, input parser, SQLite stats.
  wordle-play/    Local binary for playing (and for the dev loop).
assets/words/     Allowed-guess dictionary and fallback answers.
deploy/           Dockerfile, compose, systemd unit.
docs/             Architecture and deployment notes.
```

## Development

```bash
git clone https://github.com/GameboyColor32/wordle-tui
cd wordle-tui

cargo run -p wordle-play          # play locally, fastest feedback loop
cargo test --workspace            # unit + snapshot tests
cargo clippy --workspace -- -D warnings
cargo fmt --all

cargo run -p wordle-ssh           # server on 127.0.0.1:2222
ssh -p 2222 localhost             # connect to it
```

Set `WORDLE_ANSWER=CRANE` to pin the puzzle while developing, so you're not
debugging against a word you don't know.

## Self-hosting

```bash
docker compose -f deploy/compose.yml up -d
```

The host key lives on a mounted volume and must persist across deploys —
regenerating it greets every returning player with SSH's host-key-changed
warning. Full notes in [docs/DEPLOY.md](docs/DEPLOY.md).

## Contributing

Issues and PRs welcome. New behaviour in `wordle-core` should come with a test;
UI changes should come with an updated snapshot.

## Acknowledgements

- [ratatui](https://ratatui.rs) for the TUI framework
- [russh](https://github.com/Eugeny/russh) for the SSH server
- Josh Wardle, for making the thing in the first place

## Disclaimer

This is an unofficial, non-commercial hobby project. It is not affiliated with,
endorsed by, or connected to The New York Times. It reads the same public
endpoint your browser does, once per day. *Wordle* is a trademark of The New York
Times Company.

## License

[MIT](LICENSE)

# wordle-tui

A terminal Wordle that fetches the daily puzzle and caches it locally, with a
bundled word list as an offline fallback.

## Usage

```bash
# Run locally
cargo run --package wordle-play

# Or run with Docker
docker build --tag wordle-tui .
docker run --rm --interactive --tty \
  --volume wordle-cache:/home/wordle/cache \
  wordle-tui
```

## Third-party services and data

- Daily puzzles are requested from the public New York Times Wordle endpoint.
  This project is unofficial and is not affiliated with or endorsed by The New
  York Times. *Wordle* is a trademark of The New York Times Company.
- The bundled fallback dictionary is derived from
  [Tab Atkins' `wordle-list`](https://github.com/tabatkins/wordle-list),
  copyright © 2022 Tab Atkins Jr., distributed under the
  [MIT License](https://github.com/tabatkins/wordle-list/blob/main/LICENSE).

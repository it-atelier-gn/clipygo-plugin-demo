# clipygo-plugin-demo

[![Build](https://github.com/it-atelier-gn/clipygo-plugin-demo/actions/workflows/ci.yml/badge.svg)](https://github.com/it-atelier-gn/clipygo-plugin-demo/actions)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A demo plugin for [clipygo](https://github.com/it-atelier-gn/clipygo). Exposes four dummy targets that log received content to stderr and respond with success. Also demonstrates the optional config protocol so you can see how the settings UI works.

Use this as a starting point for writing your own plugin. See the [plugin protocol docs](https://github.com/it-atelier-gn/clipygo/blob/main/docs/plugins.md) for the full spec.

## Supported formats

- **Demo Target 1** — text
- **Demo Target 2** — text, image (saves the image to a temp file and opens it)
- **Demo Target 3 (HTML)** — html, text (saves the HTML to a temp file and opens it)
- **Demo Target 4 (Files)** — files (logs the received file paths to stderr)

## Building

```sh
cargo build --release
```

## Installing

Either download a pre-built binary from [Releases](https://github.com/it-atelier-gn/clipygo-plugin-demo/releases), or install directly from the plugin registry in clipygo's Settings.

To register manually: Settings → Plugins → add the path to the binary as the command.

## License

MIT © 2026 Georg Nelles

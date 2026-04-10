# clipygo-plugin-demo

[![Build](https://github.com/it-atelier-gn/clipygo-plugin-demo/actions/workflows/ci.yml/badge.svg)](https://github.com/it-atelier-gn/clipygo-plugin-demo/actions)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A demo plugin for [clipygo](https://github.com/it-atelier-gn/clipygo). Exposes two dummy targets that log received content to stderr and respond with success. Also demonstrates the optional config protocol so you can see how the settings UI works.

Use this as a starting point for writing your own plugin. See the [plugin protocol docs](https://github.com/it-atelier-gn/clipygo/blob/main/docs/plugins.md) for the full spec.

## Building

```sh
cargo build --release
```

## Installing

Either download a pre-built binary from [Releases](https://github.com/it-atelier-gn/clipygo-plugin-demo/releases), or install directly from the plugin registry in clipygo's Settings.

To register manually: Settings → Plugins → add the path to the binary as the command.

## License

MIT © 2026 Georg Nelles

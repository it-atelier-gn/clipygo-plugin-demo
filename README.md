# clipygo-plugin-demo

A demo subprocess target provider plugin for [clipygo](https://github.com/it-atelier-gn/clipygo).

## What it does

This plugin exposes two dummy targets (`Demo Target 1`, `Demo Target 2`). When clipygo sends content to one of them, the plugin logs it to stderr and responds with `success: true`. It serves as a minimal working reference implementation of the clipygo plugin protocol.

## Plugin protocol

Plugins communicate with clipygo over **stdin/stdout** using newline-delimited JSON.

### Requests (clipygo → plugin)

**`get_info`** — called once at startup to identify the plugin:
```json
{"command":"get_info"}
```

**`get_targets`** — fetch the list of targets this plugin provides:
```json
{"command":"get_targets"}
```

**`send`** — deliver clipboard content to a target:
```json
{"command":"send","target_id":"demo-target-1","content":"Hello world","format":"text"}
```

### Responses (plugin → clipygo)

Each response is a single JSON object on one line.

**`get_info` response:**
```json
{"name":"Demo Plugin","version":"1.0.0","description":"...","author":"..."}
```

**`get_targets` response:**
```json
{
  "targets": [
    {
      "id": "demo-target-1",
      "provider": "Demo Plugin",
      "formats": ["text"],
      "title": "Demo Target 1",
      "description": "First demo target",
      "image": "<base64-encoded PNG>"
    }
  ]
}
```

**`send` response:**
```json
{"success":true}
```

On error:
```json
{"success":false,"error":"reason"}
```

## Building

```sh
cargo build --release
```

The binary is at `target/release/clipygo-plugin-demo` (or `.exe` on Windows).

## Releases

Pre-built binaries for Windows, Linux, and macOS are published automatically via GitHub Actions on every `v*` tag.

| Platform | Artifact |
|---|---|
| Windows x64 | `clipygo-plugin-demo-windows-x64.exe` |
| Linux x64 | `clipygo-plugin-demo-linux-x64` |
| macOS ARM64 | `clipygo-plugin-demo-macos-arm64` |

SHA256 checksums are published alongside each binary.

## Registering in clipygo

In clipygo Settings → Plugins, add the path to the downloaded binary as the command. Or install it directly from the [clipygo plugin registry](https://github.com/it-atelier-gn/clipygo-plugins).

# clipygo-plugin-demo

A demo subprocess target provider plugin for [clipygo](https://github.com/it-atelier-gn/clipygo).

## What it does

This plugin exposes two dummy targets (`Demo Target 1`, `Demo Target 2`). When clipygo sends content to one of them, the plugin logs it to stderr and responds with `success: true`. It also demonstrates the optional configuration protocol (including setup instructions and a link to this repo), letting users adjust settings through clipygo's plugin config UI. It serves as a minimal working reference implementation of the clipygo plugin protocol.

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

**`get_config_schema`** *(optional)* — return a JSON Schema describing the plugin's configurable settings, along with current values and setup instructions:
```json
{"command":"get_config_schema"}
```

**`set_config`** *(optional)* — apply new configuration values:
```json
{"command":"set_config","values":{"greeting":"Hey!","verbose":true}}
```

### Responses (plugin → clipygo)

Each response is a single JSON object on one line.

**`get_info` response:**
```json
{"name":"Demo Plugin","version":"1.3.0","description":"...","author":"...","link":"https://github.com/..."}
```

The optional `link` field provides a URL (e.g. repo page) shown next to the plugin name in settings.

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

**`get_config_schema` response:**
```json
{
  "instructions": "Plain-text setup instructions shown above the config fields.",
  "schema": {
    "type": "object",
    "title": "Demo Plugin",
    "properties": {
      "greeting": {
        "type": "string",
        "title": "Greeting Message",
        "description": "Message logged when content is sent to a target",
        "default": "Received!"
      },
      "verbose": {
        "type": "boolean",
        "title": "Verbose Logging",
        "description": "Log full content to stderr (not just a preview)"
      }
    }
  },
  "values": {
    "greeting": "Received!",
    "verbose": false
  }
}
```

The optional `instructions` field is displayed above the config fields in the settings UI.

**`set_config` response:**
```json
{"success":true}
```

Supported property types: `string`, `boolean`. Strings with `"format": "password"` render as password fields. Properties with `"enum"` render as dropdowns. Properties with `"visibleIf"` are conditionally shown based on another field's value.

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

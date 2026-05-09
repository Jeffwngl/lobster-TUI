# lobster-TUI
A TUI for the Lobste.rs website.

![picture]("public/ui.png")

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratatui` | 0.30.0 | Terminal UI framework — layouts, widgets, rendering |
| `crossterm` | 0.29.0 | Cross-platform terminal input and raw mode control |
| `tokio` | 1.52.2 | Async runtime for non-blocking network requests |
| `reqwest` | 0.13.3 | HTTP client for fetching Lobsters API and article HTML |
| `serde` | 1.0.228 | Serialization framework — derives `Deserialize` on structs |
| `anyhow` | 1.0.102 | Ergonomic error handling and propagation |
| `readable-readability` | 0.4.0 | Extracts main article content from raw HTML |
| `html5ever` | 0.39.0 | HTML parser used internally by readable-readability |
| `url` | 2.5.8 | URL parsing for passing base URLs to readability |
| `open` | 5.3.4 | Opens URLs in the system default browser |

## Build from Source

Run the following command in the terminal:
```
git clone https://github.com/Jeffwngl/lobster-TUI.git

cd lobster-TUI

cargo build --release
```
Then move the executable to your `$PATH`.

## Controls

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Enter` | Open story in browser |
| `c` | Open comments in browser |
| `v` | View comments in TUI |
| `o` | View article in TUI |
| `Esc` | Back to stories |
| `q` | Quit |

## Future Development

- Add improved html rendering.
- Add mouse controls.
- Improve customization options.

## License

MIT
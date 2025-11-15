# nix-terminal

Tellurian's experimental *nix terminal shell built with [iced](https://github.com/iced-rs/iced).
The current bootstrap provides a cross-platform Rust application with an Iced window,
a scrollable output view, and a prompt/input row that will later be wired to the
terminal backend.

## Getting started

```
rustup target add wasm32-unknown-unknown # optional, for future builds
cargo run
```

That command launches the development window. Typed commands are echoed into the
scrollback to mimic a terminal session while the real execution engine is being
implemented.

## Project layout

```
.
├── Cargo.toml       # crate metadata and UI dependencies (iced)
├── README.md        # onboarding and quickstart instructions
└── src
    ├── app.rs       # Iced Application implementation (UI state + view)
    └── main.rs      # Program entrypoint configuring window + settings
```

## Next steps

* Hook an actual pseudo-terminal backend into `NixTerminalApp::update`.
* Introduce domain modules (renderer, compositor, shell integration, settings).
* Expand the UI theme, color palette, and window controls.

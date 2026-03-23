# VIO Launcher

VIO Launcher is a Rust workspace for a Minecraft launcher with a desktop GUI built on Tauri and Vue.

## Workspace
- `theseus` - launcher core library and APIs
- `theseus_gui/src-tauri` - Tauri desktop shell
- `theseus_gui/src` - Vue frontend
- `theseus_playground` - local playground binary
- `theseus_macros` - procedural macros used by the core

## Build
### Windows
```bat
build.bat
```

### Manual release build
```bash
cargo build --release
```

## Outputs
- `target/release/vio_gui.exe`
- `target/release/vio_playground.exe`

## Releases
https://github.com/VIO-Launcher/VIO/releases

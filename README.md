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
https://github.com/Jodahh1/VIO-Launcher/releases

## Cross-platform installers
GitHub Actions builds release installers for:
- Windows (`.msi`)
- Linux (`.deb`, `.AppImage`)
- macOS (`.app`, `.dmg`)

Create and push a version tag to trigger the release workflow:

```bash
git tag v0.7.2
git push origin v0.7.2
```

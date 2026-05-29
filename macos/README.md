# macOS

## Prerequisites

```bash
brew install luajit luarocks
luarocks --lua-dir $(brew --prefix luajit) install Lua-cURLv3 luasocket luautf8
cargo install cargo-packager --locked
```

## Build

```bash
cargo packager --release --config macos/Packager-poe1.toml --formats app
cargo packager --release --config macos/Packager-poe2.toml --formats app
```

This produces `target/release/Path of Building.app` and `target/release/Path of Building 2.app`.

# Tauri Plugin Mqtt

This plugin provides an interface with [rumqtt](https://github.com/bytebeamio/rumqtt).

# **NOTE:** This is in heavy development and does not have even basic functionality yet.

## Architecture
This repo shape might appear to be strange, but it is really just a hybrid Rust / Typescript project that recommends a specific type of consumption, namely using GIT as the secure distribution mechanism, and referencing specific unforgeable git hashes. ~~ Of course, it can also be consumed via Cargo and NPM.~~

### `/src`
Rust source code that contains the plugin definition.

### `/webview-src`
Typescript source for the /webview-dist folder that provides an API to interface with the rust code.

### `/webview-dist`
Tree-shakeable transpiled JS to be consumed in a Tauri application.

## Installation
There are three general methods of installation that we can recommend.
1. Pull sources directly from Github using git tags / revision hashes (most secure, good for developement, shown below)
2. Git submodule install this repo in your tauri project and then use `file` protocol to ingest the source
3. ~~Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)~~


### RUST
`src-tauri/Cargo.toml`
```yaml
[dependencies.tauri-plugin-mqtt]
git = "https://github.com/TheBestJohn/tauri-plugin-mqtt"
tag = "v0.1.0"
#branch = "main"
```

Use in `src-tauri/src/main.rs`:
```rust
use tauri_plugin_mqtt::init;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_mqtt::init())
        .build()
        .run();
}
```

### WEBVIEW
`Install from a tagged release`
```
npm install github:TheBestJohn/tauri-plugin-mqtt#v0.1.0
# or
yarn add github:TheBestJohn/tauri-plugin-mqtt#v0.1.0
```

`Install from a commit`
```
npm install github:TheBestJohn/tauri-plugin-mqtt#488558717b77d8a2bcb37acfd2eca9658aeadc8e
# or
yarn add github:TheBestJohn/tauri-plugin-mqtt#488558717b77d8a2bcb37acfd2eca9658aeadc8e
```

`package.json`
```json
  "dependencies": {
    "tauri-plugin-mqtt": "github:TheBestJohn/tauri-plugin-mqtt#v0.1.0",
```

Use within your JS/TS:
```ts
IN DEV
```

# License
MIT / Apache-2.0

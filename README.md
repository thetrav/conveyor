# Conveyor game

This is all Rahn's idea, you can blame him I'm just the code monkey!

# desktop mode

## watch mode
```
cargo watch -q -c -x 'run --features bevy/dynamic'
```

# wasm mode
trying to get this working for all the browser kids

## dependencies
You will need a few things installed on the dev machine:

https://bevy-cheatbook.github.io/platforms/wasm.html
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
```

https://rustwasm.github.io/wasm-bindgen/reference/cli.html
```
cargo install -f wasm-bindgen-cli
```

## Local running
```
cargo run --target wasm32-unknown-unknown
```

## Bundle game for release
```
cargo build --release --target wasm32-unknown-unknown
```

## Create web assets
The following command will create js and wasm 
```
wasm-bindgen --no-typescript --out-dir out --target web .\target\wasm32-unknown-unknown\release\conveyor.wasm
```

## View locally
```
python -m http.server
```
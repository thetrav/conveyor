# Conveyor game

This is all Rahn's idea, you can blame him I'm just the code monkey!

# desktop mode

## watch mode
```
cargo watch -q -c -x 'run --features bevy/dynamic_linking'
```

## run

```
cargo run --features bevy/dynamic_linking
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
note: currently fails on my windows box complaining about missing c compiler dependencies but works on the github actions toolchain which is good enough for me
```
cargo build --release --target wasm32-unknown-unknown --features = tiled/wasm
```

## Create web assets
The following command will create js and wasm 
```
wasm-bindgen --no-typescript --out-dir wasm --target web .\target\wasm32-unknown-unknown\release\conveyor.wasm
```

## View locally
not really set up to do this easily, but if you copy the assets tree into the wasm folder you can run the game via a local http server


```
python -m http.server
```

## itch.io

Deployed as draft currently, if you really want to try it here's a secret URL:

https://the-trav.itch.io/conveyor?secret=mHY5xzc2ZragwWTbl8C0X8Tr8

It's more set up for auto deployment to itch.io via butler.
For my own memory sake, I had to:

1. First create the project at itch.io
2. Run a local docker container containing butler 
3. Do the login so it registered its own API key, (that's probably against my account and I can use for future projects)
4. Then I put that key in the github secrets for the action
5. Run the butler push
6. go to the itch project page, edit the game, set the channel I uploaded to html5 play in browser

# Weird pixel borders on tiles

Related to https://github.com/bevyengine/bevy/issues/8150 the bevy texture atlas seems to have problematic sampling at the edges.

I've found that if I use tile maps with any spacing, at some zoom levels the renderer picks the wrong pixel and you get the padding from the tile map showing through making your game look like it's gone through a wire mesh.

There's some talk about MSAA fixing it, however I haven't had any luck there.  Using individual sprite files also could fix it, but there are warnings around doing that with wasm deployments
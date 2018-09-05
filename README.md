# dots
WIP clone of [Boomshine](http://www.k2xl.com/games/boomshine/). The original requires Flash, so it's likely unplayable - an [Android app](https://play.google.com/store/apps/details?id=com.bantambytes.android.game.boomshine&hl=en_US) exists as well.

The logic is implemented in Rust targeting WebAssembly, with the rendering handled by JavaScript to a `canvas` element.

Currently, not much is implemented - there's no levels or endgame detection, you can just click the screen once to try to capture as many as you can.  To try again, click "Restart".

![screenshot](https://i.imgur.com/zZD0hiI.png)

## Usage

Stay tuned, I plan to self-host the finished app.

## Develop

Requires Rust nightly and `wasm-pack`. 

1. Clone this repo
2. Execute `wasm-pack init` in the project root
3. Execute `yarn link` in the newly generated `pkg` dir to make the WASM module available to the frontend
4. Execute `yarn link dots` in the `www` dir
5. If this is the first run, execute `yarn install` in the `www` dir
6. Execute `yarn start` from within `www`.  This will serve the app on `localhost:8080`

Any changes to the JS in `www/index.js` will be picked up by webpack, but if you change any Rust code you need to re-invoke `wasm-pack init`.

## Acknowledgements

This is pretty much the tutorial from the RustWasm [book](https://rustwasm.github.io/book/) with a bigger game behind it.  Thanks, RustWasm crew!
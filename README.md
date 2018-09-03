# dots
WIP clone of [Boomshine](http://www.k2xl.com/games/boomshine/). The original requires Flash, so it likely unplayable - an [Android app](https://play.google.com/store/apps/details?id=com.bantambytes.android.game.boomshine&hl=en_US) exists as well.

The logic implemented in Rust targeting WebAssembly, with the rendering handled by JavaScript to a `canvas` element.

Currently, not much is implemented - there's no levels or endgame detection, you can just click the screen once to try to capture as many as you can.  To try again, click "Restart".

## Usage

Stay tuned, I plan to self-host the finish app.

## Develop

Requires Rust nightly and `wasm-pack`.  Clone this repo, execute `wasm-pack init` in the project root, and in a separate terminal execute `yarn start` from within `dots/www`.  This will serve the app on `localhost:8080`.
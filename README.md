# dots
Clone of [Boomshine](http://www.k2xl.com/games/boomshine/).  This version can be played [here](http://deciduously.com/dots)

The logic is implemented in Rust targeting WebAssembly, with the rendering handled by JavaScript to a `canvas` element.

![screenshot](https://i.imgur.com/QYgJVLW.png)

## Usage

Play the current release on [deciduously.com](http://deciduously.com/static/extern/dots/index.html).

## Develop

Requires Rust nightly and `wasm-pack`. 

1. Clone this repo
2. Execute `wasm-pack build` in the project root
3. Execute `yarn link` in the newly generated `pkg` dir to make the WASM module available to the frontend
4. Execute `yarn link dots` in the `www` dir
5. If this is the first run, execute `yarn install` in the `www` dir
6. Execute `yarn start` from within `www`.  This will serve the app on `localhost:8080`

For future rebuilds, you only need step 2 every time you change the Rust code and 6 just once to start the dev server.

Compile with `--no-default-features` to disable the console error debugging machinery for deployment.

## Acknowledgements

The [rustwasm book](https://rustwasm.github.io/book/) was a great kicking off point.
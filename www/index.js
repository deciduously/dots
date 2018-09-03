import { Game } from "dots";
import { networkInterfaces } from "os";

// capping the tick to 60 times a second
const updates_per_second = 60;
const millis_per_update = 1 / updates_per_second * 1000;

// start things up - Game is our WASM interface
const game = Game.new();

// set up the render context
const canvas  = document.getElementById("dots-canvas");
const height = game.height();
const width = game.width();
canvas.height = height;
canvas.width = width;
const ctx = canvas.getContext('2d');

// click handler
canvas.addEventListener("click", event => {
    // translate from page coords to canvas coords
    // shamelessly lifted from the RustWasm book
    // https://rustwasm.github.io/book/game-of-life/interactivity.html
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
  
    const canvasX = (event.clientX - boundingRect.left) * scaleX;
    const canvasY = (event.clientY - boundingRect.top) * scaleY;

    game.add_player(canvasX, canvasY);
});

// define the main loop, updated 60 times per second
const renderLoop = () => {
    if (Date.now() - game.last_update() >= millis_per_update) {
        game.tick()
        drawGame();
        requestAnimationFrame(renderLoop);
    }
}

// Define how to draw a single frame
const drawGame = () => {
    const numDots = game.num_dots();
    ctx.clearRect(0, 0, width, height);

    // draw each dot, grabbing params from the WASM
    for (let idx = 0; idx < numDots; idx++) {
        if (game.draw_dot(idx)) {
            ctx.beginPath();
            // use an arc from 0 to 2pi to draw a full circle
            ctx.arc(game.get_dot_x(idx), game.get_dot_y(idx), game.get_dot_radius(idx), 0, 2 * Math.PI, false);
            ctx.fillStyle = game.get_dot_color(idx);
            ctx.fill();
            ctx.stroke();
        }
    }
}

// Kick off the render loop by asking for the first frame
requestAnimationFrame(renderLoop);
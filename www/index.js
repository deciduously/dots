import { Game } from "dots";
import { memory } from "dots/dots_bg";

// start things up - Game is our WASM interface
const game = Game.new();

// set up the render context
const canvas  = document.getElementById("dots-canvas");
const height = game.height();
const width = game.width();
canvas.height = height;
canvas.width = width;
const ctx = canvas.getContext('2d');

// define the main loop
const renderLoop = () => {
    game.tick()

    drawGame();

    requestAnimationFrame(renderLoop);
}

// Define how to draw a single frame
const drawGame = () => {
    const numDots = game.num_dots();
    ctx.clearRect(0, 0, width,  height);

    // draw each dot, grabbing params from the WASM
    for (let idx = 0; idx < numDots; idx++) {
        ctx.beginPath();
        // use an arc from 0 to 2pi to draw a full circle
        ctx.arc(game.get_dot_x(idx), game.get_dot_y(idx), game.get_dot_radius(idx), 0, 2 * Math.PI, false);
        ctx.fillStyle = game.get_dot_color(idx);
        ctx.fill();
        ctx.stroke();
    }
}

// Kick off the render loop by asking for the first frame
requestAnimationFrame(renderLoop);
import { Game } from "dots";
import { memory } from "dots/dots_bg";

const game = Game.new();

const canvas  = document.getElementById("dots-canvas");
canvas.height = game.height();
canvas.width = game.width();

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    game.tick()

    drawGame();

    requestAnimationFrame(renderLoop);
}

const drawGame = () => {
    const dotsPtr = game.dots();
    const numDots = game.num_dots();
    const dots = new Array(memory.buffer, dotsPtr, numDots);

    ctx.beginPath();

    for (let idx = 0; idx < numDots; idx++) {
        ctx.arc(game.get_dot_x(idx), game.get_dot_y(idx), game.get_dot_radius(idx), 0, 2 * Math.PI, false);
        ctx.fillStyle = 'green';
        ctx.fill();
    }

    ctx.stroke();
}

requestAnimationFrame(renderLoop);
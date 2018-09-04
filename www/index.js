import { PackedDotState, Game } from 'dots'
import { memory } from 'dots/dots_bg'

// capping the tick to 60 times a second
const updatesPerSecond = 60
const millisPerUpdate = 1 / updatesPerSecond * 1000

// start things up - Game is our WASM interface
const game = new Game(40)

// set up the render context
const canvas = document.getElementById('dots-canvas')
const height = game.height()
const width = game.width()
canvas.height = height
canvas.width = width
const ctx = canvas.getContext('2d')
ctx.globalAlpha = 0.8 // everything's a little transparent

// Restart button - TODO have it flip and be the same button as start-game
const restartButton = document.getElementById('restart-button')
restartButton.addEventListener('click', event => {
  game.load_level(40) // eventually 1,2,3etc, for now its num_dots
})

// Canvas click handler
canvas.addEventListener('click', event => {
  // translate from page coords to canvas coords
  // shamelessly lifted from the RustWasm book
  // https://rustwasm.github.io/book/game-of-life/interactivity.html
  const boundingRect = canvas.getBoundingClientRect()

  const scaleX = canvas.width / boundingRect.width
  const scaleY = canvas.height / boundingRect.height

  const canvasX = (event.clientX - boundingRect.left) * scaleX
  const canvasY = (event.clientY - boundingRect.top) * scaleY

  game.add_player(canvasX, canvasY)
})

// define the main loop, updated 60 times per second
const renderLoop = () => {
  if (Date.now() - game.last_update() >= millisPerUpdate) {
    game.tick()
    drawGame()
    window.requestAnimationFrame(renderLoop)
  }
}

// Define how to draw a single frame
const drawGame = () => {

  // Start with a blank slate
  ctx.clearRect(0, 0, width, height)

  // Get our dots
  const dotsPtr = game.pack()
  const dots = new Float32Array(memory.buffer, dotsPtr, game.numDots())

  // Draw the progress counter
  ctx.font = '32px serif'
  ctx.fillStyle = 'red'
  ctx.fillText(game.get_progress_text(), 10, 42)

  // draw each dot, grabbing params from the WASM
  for (let idx = 0; idx < numDots; idx++) {
    // We're getting a packed [f32; 10]: id | x | y | radius | t_x | t_y | DotState | r | g | b
    if (dots[idx][6] != PackedDotState.Dead) {
      const pos_x = dots[idx][1]
      const pos_y = dots[idx][2]
      const radius = dots[idx][3]
      const color = colorString(dots[idx][7], [idx][8], dots[idx][9])
      ctx.beginPath()
      // use an arc from 0 to 2pi to draw a full circle
      ctx.arc(pos_x, pos_y, radius, 0, 2 * Math.PI, false)
      ctx.fillStyle = color
      ctx.fill()
      ctx.stroke()
    }
  }
}

const colorString = (r, g, b) => {
  "#" + r.toString(16) + g.toString(16) + b.toString(16)
}

// Kick off the render loop by asking for the first frame
window.requestAnimationFrame(renderLoop)

game.free()
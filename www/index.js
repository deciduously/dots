import { Game } from 'dots'
import { memory } from 'dots/dots_bg'

// capping the tick to 60 times a second
const updatesPerSecond = 60
const millisPerUpdate = 1 / updatesPerSecond * 1000

var game = new Game()

// set up the render context
const canvas = document.getElementById('dots-canvas')
const height = game.height()
const width = game.width()
canvas.height = height
canvas.width = width
const ctx = canvas.getContext('2d')
ctx.globalAlpha = 0.8 // everything's a little transparent

// Restart game button
const restartGameButton = document.getElementById('restart-game-button')
restartGameButton.addEventListener('click', event => {
  game = new Game()
})

// Restart level button
const restartLevelButton = document.getElementById('restart-level-button')
restartLevelButton.addEventListener('click', event => {
  game.restart_level()
})

// Next level button
const nextLevelButton = document.getElementById('next-level-button')
nextLevelButton.addEventListener('click', event => {
  game.next_level()
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
  // tick us forward and grab the packed version
  game.tick()
  const levelPtr = game.pack()

  // read header
  // level_number | total_dots | win_threshold | captured_dots | last_update
  const levelData = new Float32Array(memory.buffer, levelPtr, 5)
  const level = levelData[0]
  const totalDots = levelData[1]
  const winThreshold = levelData[2]
  const capturedDots = levelData[3]
  const lastUpdate = levelData[4]

  // get dots
  const dataLength = totalDots * 7 + 5
  const dots = new Float32Array(memory.buffer, levelPtr, dataLength).slice(5, dataLength + 1)

  if (Date.now() - lastUpdate >= millisPerUpdate) {
    drawGame(dots, level, totalDots, winThreshold, capturedDots)
    window.requestAnimationFrame(renderLoop)
  }
}

// Define how to draw a single frame
const drawGame = (dots, level, totalDots, winThreshold, capturedDots) => {
  // Start with a blank slate
  ctx.clearRect(0, 0, width, height)

  // to tell if we won
  const won = capturedDots >= winThreshold

  // Draw the progress counter
  ctx.font = '32px serif'
  ctx.fillStyle = won ? 'green' : 'red'
  ctx.fillText(capturedDots + '/' + totalDots, 10, 42) // this will be wrong until I implement appstate - its including the player dot

  // Draw the level number
  ctx.font = '20px serif'
  ctx.fillStyle = 'blue'
  ctx.fillText('level ' + level, 10, 70)

  // draw each dot
  let dotsLength = dots.length
  for (let idx = 0; idx < dotsLength; idx += 7) {
    // We're getting a packed [f32; 7]:  x | y | radius | DotState | r | g | b
    if (dots[idx + 3] !== 5.0) {
      const posX = dots[idx]
      const posY = dots[idx + 1]
      const radius = dots[idx + 2]
      const color = colorString(dots[idx + 4], dots[idx + 5], dots[idx + 6])
      // console.log('(' + posX + 'y' + posY + ' r: ' + radius + ' color: ' + color)
      ctx.beginPath()
      // use an arc from 0 to 2pi to draw a full circle
      ctx.arc(posX, posY, radius, 0, 2 * Math.PI, false)
      ctx.fillStyle = color
      ctx.fill()
      ctx.stroke()
    }
  }
}

const colorString = (r, g, b) => '#' + Math.floor(r).toString(16) + Math.floor(g).toString(16) + Math.floor(b).toString(16)

// Kick off the render loop by asking for the first frame
window.requestAnimationFrame(renderLoop)

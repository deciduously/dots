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
  game.free()
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
  // level_number | level_state | total_dots | win_threshold | captured_dots | last_update
  const levelData = new Float32Array(memory.buffer, levelPtr, 6)
  const level = levelData[0]
  const levelState = levelData[1]
  const totalDots = levelData[2]
  const winThreshold = levelData[3]
  const capturedDots = levelData[4]
  const lastUpdate = levelData[5]

  // get dots
  const dataLength = totalDots * 7 + 6
  const dots = new Float32Array(memory.buffer, levelPtr, dataLength).slice(6)

  if (Date.now() - lastUpdate >= millisPerUpdate) {
    drawGame(dots, level, totalDots, winThreshold, capturedDots)
    window.requestAnimationFrame(renderLoop)
  }
}

// DRAW FNS

// Define how to draw a single frame
const drawGame = (dots, level, totalDots, winThreshold, capturedDots) => {
  // Start with a blank slate
  ctx.clearRect(0, 0, width, height)

  drawProgressCounter(capturedDots, totalDots, winThreshold)
  drawLevelNumber(level)

  let dotsLength = dots.length
  for (let idx = 0; idx < dotsLength; idx += 7) {
    drawDot(dots.slice(idx, idx + 7))
  }
}

const drawProgressCounter = (capturedDots, totalDots, winThreshold) => {
  const won = capturedDots >= winThreshold
  ctx.font = '32px serif'
  ctx.fillStyle = won ? 'green' : 'red'
  ctx.fillText(capturedDots + '/' + totalDots, 10, 42) // this will be wrong until I implement appstate - its including the player dot
}

const drawLevelNumber = level => {
  ctx.font = '20px serif'
  ctx.fillStyle = 'blue'
  ctx.fillText('level ' + level, 10, 70)
}

const drawDot = packedDot => {
  // x | y | radius | DotState | r | g | b
  if (packedDot[3] !== 5.0) {
    const posX = packedDot[0]
    const posY = packedDot[1]
    const radius = packedDot[2]
    const color = colorString(packedDot[4], packedDot[5], packedDot[6])

    ctx.beginPath()
    // use an arc from 0 to 2pi to draw a full circle
    ctx.arc(posX, posY, radius, 0, 2 * Math.PI, false)
    ctx.fillStyle = color
    ctx.fill()
    ctx.stroke()
  }
}

const colorString = (r, g, b) => '#' + Math.floor(r).toString(16) + Math.floor(g).toString(16) + Math.floor(b).toString(16)

// INIT

window.requestAnimationFrame(renderLoop)

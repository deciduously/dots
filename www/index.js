import { Game } from 'dots'
import { memory } from 'dots/dots_bg'

var game = new Game()

// set up the render context
const canvas = document.getElementById('dots-canvas')
const height = game.height()
const width = game.width()
canvas.height = height
canvas.width = width
const ctx = canvas.getContext('2d')
ctx.globalAlpha = 0.8 // everything's a little transparent

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

  game.handle_click(canvasX, canvasY)
})

// define the main loop, updated 60 times per second
const renderLoop = () => {
  // Start with a blank slate
  ctx.clearRect(0, 0, width, height)

  // tick us forward and grab the packed version
  game.tick()
  const headerPtr = game.header()

  // read header
  // level_number | level_state | total_dots | win_threshold | captured_dots
  const levelData = new Uint8Array(memory.buffer, headerPtr, 5)
  const level = levelData[0]
  const levelState = levelData[1]
  const totalDots = levelData[2]
  const winThreshold = levelData[3]
  const capturedDots = levelData[4]

  // LevelState:
  // Begin = 0,
  // Waiting = 1,
  // Clicked = 2,
  // Won = 3,
  // Lost = 4

  switch (levelState) {
    case 0: {
      drawBeginLevel(level, winThreshold, totalDots)
      window.requestAnimationFrame(renderLoop)
      break
    }
    case 1:
    case 2: {
      // get dots
      const dataLength = totalDots * 7
      const dotsPtr = game.pack()
      const dots = new Float32Array(memory.buffer, dotsPtr, dataLength)

      drawGame(dots, level, totalDots, winThreshold, capturedDots, levelState)
      window.requestAnimationFrame(renderLoop)
      break
    }
    case 3: {
      drawNextLevel(level)
      window.requestAnimationFrame(renderLoop)
      break
    }
    case 4: {
      drawRestartLevel(level)
      window.requestAnimationFrame(renderLoop)
      break
    }
    default: { }
  }
}

// DRAW FNS

const drawLevelButton = (text, colorStr) => {
  ctx.beginPath()
  ctx.rect(325, 275, 150, 50)
  ctx.stroke()
  ctx.font = '18px serif'
  ctx.fillStyle = colorStr
  ctx.fillText(text, 330, 305)
}

const drawBeginLevel = (level, winThreshold) => {
  drawLevelButton('Level ' + level + ' - capture ' + winThreshold, 'purple')
}

const drawRestartLevel = (level) => {
  drawLevelButton('Too bad! Retry ' + level, 'red')
}

const drawNextLevel = (level) => {
  drawLevelButton('Nice job! Level ' + (level + 1), 'green')
}

const drawGame = (dots, level, totalDots, winThreshold, capturedDots, levelState) => {
  drawProgressCounter(capturedDots, totalDots, winThreshold, levelState)
  drawLevelNumber(level)

  let dotsLength = dots.length
  for (let idx = 0; idx < dotsLength; idx += 7) {
    drawDot(dots.slice(idx, idx + 7))
  }
}

const drawProgressCounter = (capturedDots, totalDots, winThreshold, levelState) => {
  const won = capturedDots >= winThreshold
  const levelDots = (levelState == 1) ? totalDots : totalDots - 1
  ctx.font = '22px serif'
  ctx.fillStyle = won ? 'green' : 'red'
  ctx.fillText(capturedDots + '/' + levelDots + ' - goal: ' + winThreshold, 10, 42) // this will be wrong until I implement appstate - its including the player dot
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

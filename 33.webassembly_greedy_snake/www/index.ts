import init, { Game, Direction, Index, Status } from 'greedy_snake_wasm'
import wasmPath from 'greedy_snake_wasm/greedy_snake_wasm_bg.wasm?url'

const start = document.getElementById('start') as HTMLButtonElement
const scoreContainer = document.getElementById('scoreContainer') as HTMLDivElement
const score = document.getElementById('score') as HTMLSpanElement
const canvas = document.querySelector('canvas') as HTMLCanvasElement
const ctx = canvas.getContext('2d') as CanvasRenderingContext2D

const cellSize = 18
const cellCount = 40
const defaultDirection = Direction.Right
const defaultLength = 3
const target = 50

const wasm = await init(wasmPath)

let game: Game
let canvasSize: number

start.addEventListener('click', () => {
  start.innerHTML = 'Restart'
  canvas.blur()
  run()
})

window.addEventListener('keydown', e => {
  switch (e.code) {
    case 'ArrowLeft':
      game.turn(Direction.Left)
      break
    case 'ArrowRight':
      game.turn(Direction.Right)
      break
    case 'ArrowUp':
      game.turn(Direction.Up)
      break
    case 'ArrowDown':
      game.turn(Direction.Down)
      break
  }
})

function drawBackground() {
  ctx.beginPath()
  ctx.strokeStyle = '#999'

  for (let i = 0; i < cellCount + 1; i++) {
    ctx.moveTo(i * cellSize, 0)
    ctx.lineTo(i * cellSize, canvasSize)
  }

  for (let i = 0; i < cellCount + 1; i++) {
    ctx.moveTo(0, i * cellSize)
    ctx.lineTo(canvasSize, i * cellSize)
  }

  ctx.stroke()
}

function drawSnake() {
  const bodyIndices = new Uint32Array(
    wasm.memory.buffer,
    game.snake_body_indices(),
    game.snake_len(),
  )

  bodyIndices.forEach((index, i) => {
    const coord = Index.new(index).to_coordinate(cellCount)
    ctx.beginPath()
    ctx.fillStyle = i === 0 ? '#333' : '#777'
    ctx.fillRect(coord[0] * cellSize, coord[1] * cellSize, cellSize, cellSize)
  })

  ctx.stroke()
}

function drawReward() {
  const rewardIndex = game.reward_index()
  const x = rewardIndex % cellCount
  const y = Math.floor(rewardIndex / cellCount)

  ctx.beginPath()
  ctx.fillStyle = 'red'
  ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize)
  ctx.stroke()
}

function draw() {
  drawBackground()
  drawSnake()
  drawReward()
}

function render() {
  setTimeout(() => {
    game.update()

    const status = game.status()
    if (status === Status.Won) {
      alert('You win! 0v0')
      return
    } else if (status === Status.Lost) {
      alert('You Lost! x_x')
      return
    }

    score.innerHTML = `${game.score()}/${target}`

    ctx.clearRect(0, 0, canvasSize, canvasSize)
    draw()

    requestAnimationFrame(render)
  }, 100)
}

async function run() {
  game = Game.new(cellSize, cellCount, defaultLength, defaultDirection, target)
  canvasSize = game.canvas_size()

  scoreContainer.style.display = ''

  canvas.width = canvasSize
  canvas.height = canvasSize

  game.start()

  requestAnimationFrame(render)
}

import init, { CanvasData, Direction, Index, Coordinate } from 'greedy_snake_wasm'
import wasmPath from 'greedy_snake_wasm/greedy_snake_wasm_bg.wasm?url'

const CELL_SIZE = 30
const CELL_COUNT = 30

const defaultDirection = Direction.Right
const defaultLength = 3

const wasm = await init(wasmPath)

const data = CanvasData.new(CELL_SIZE, CELL_COUNT, defaultLength, defaultDirection)
const canvas_size = data.canvas_size()

const canvas = document.querySelector('canvas') as HTMLCanvasElement
canvas.width = canvas_size
canvas.height = canvas_size

const ctx = canvas.getContext('2d') as CanvasRenderingContext2D

requestAnimationFrame(render)

window.addEventListener('keydown', e => {
  switch (e.code) {
    case 'ArrowLeft':
      data.turn(Direction.Left)
      break
    case 'ArrowRight':
      data.turn(Direction.Right)
      break
    case 'ArrowUp':
      data.turn(Direction.Up)
      break
    case 'ArrowDown':
      data.turn(Direction.Down)
      break
  }
})

function drawBackground() {
  for (let i = 0; i < CELL_COUNT + 1; i++) {
    ctx.moveTo(i * CELL_SIZE, 0)
    ctx.lineTo(i * CELL_SIZE, canvas_size)
  }

  for (let i = 0; i < CELL_COUNT + 1; i++) {
    ctx.moveTo(0, i * CELL_SIZE)
    ctx.lineTo(canvas_size, i * CELL_SIZE)
  }

  ctx.stroke()
}

function drawSnake() {
  const bodyIndices = new Uint32Array(
    wasm.memory.buffer,
    data.snake_body_indices(),
    data.snake_len(),
  )

  bodyIndices.forEach((index, i) => {
    const coord = Index.new(index).to_coordinate(CELL_COUNT)
    ctx.beginPath()
    ctx.fillStyle = i === 0 ? 'grey' : 'black'
    ctx.fillRect(coord[0] * CELL_SIZE, coord[1] * CELL_SIZE, CELL_SIZE, CELL_SIZE)
  })

  ctx.stroke()
}

function drawReward() {
  const rewardIndex = data.reward_index()
  const x = rewardIndex % CELL_COUNT
  const y = Math.floor(rewardIndex / CELL_COUNT)

  ctx.beginPath()
  ctx.fillStyle = 'red'
  ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE)
  ctx.stroke()
}

function draw() {
  drawBackground()
  drawSnake()
  drawReward()
}

function render() {
  setTimeout(() => {
    ctx.clearRect(0, 0, canvas_size, canvas_size)
    data.update()
    draw()

    requestAnimationFrame(render)
  }, 200)
}

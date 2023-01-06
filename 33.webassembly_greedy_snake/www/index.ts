import init, { CanvasData } from 'greedy_snake_wasm'
import wasm from 'greedy_snake_wasm/greedy_snake_wasm_bg.wasm?url'

const CELL_SIZE = 30
const CELL_COUNT = 30

await init(wasm)

const data = CanvasData.new(CELL_SIZE, CELL_COUNT, Math.round(Math.random() * CELL_COUNT ** 2))
const canvas_size = data.canvas_size()

const canvas = document.querySelector('canvas') as HTMLCanvasElement
canvas.width = canvas_size
canvas.height = canvas_size

const ctx = canvas.getContext('2d') as CanvasRenderingContext2D

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
  const headingIndex = data.heading_index()
  const x = headingIndex % CELL_COUNT
  const y = Math.floor(headingIndex / CELL_COUNT)

  ctx.beginPath()
  ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE)
  ctx.stroke()
}

function draw() {
  drawBackground()
  drawSnake()
}

function render() {
  setTimeout(() => {
    ctx.clearRect(0, 0, canvas_size, canvas_size)
    data.update()
    draw()

    requestAnimationFrame(render)
  }, 200)
}

requestAnimationFrame(render)

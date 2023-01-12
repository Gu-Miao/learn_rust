import init, { Game, Direction, Index, Status } from 'greedy_snake_wasm'
import wasmPath from 'greedy_snake_wasm/greedy_snake_wasm_bg.wasm?url'

// 页面元素
const start = document.getElementById('start') as HTMLButtonElement
const scoreContainer = document.getElementById('scoreContainer') as HTMLDivElement
const score = document.getElementById('score') as HTMLSpanElement
const canvas = document.querySelector('canvas') as HTMLCanvasElement

// Canvas 上下文
const ctx = canvas.getContext('2d') as CanvasRenderingContext2D

// 游戏参数
const cellSize = 18
const cellCount = 40
const defaultDirection = Direction.Right
const defaultLength = 3
const target = 50

// 加载 wasm
const wasm = await init(wasmPath)

/** 游戏实例，每局游戏会重新创建实例 */
let game: Game
/** 画布边长 */
let canvasSize: number

// 开始/重玩
start.addEventListener('click', () => {
  start.innerHTML = 'Restart'
  canvas.blur()
  run()
})

// 转向
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

// 渲染背景格子
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

// 渲染蛇身
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

// 渲染蛋
function drawReward() {
  const coord = game.reward_index().to_coordinate(cellCount)

  ctx.beginPath()
  ctx.fillStyle = 'red'
  ctx.fillRect(coord[0] * cellSize, coord[1] * cellSize, cellSize, cellSize)
  ctx.stroke()
}

function draw() {
  drawBackground()
  drawSnake()
  drawReward()
}

function render() {
  setTimeout(() => {
    // 更新数据
    game.update()

    // 判断游戏是否胜利/失败
    const status = game.status()
    if (status === Status.Won) {
      alert('You win! 0v0')
      return
    } else if (status === Status.Lost) {
      alert('You Lost! x_x')
      return
    }

    score.innerHTML = `${game.score()}/${target}`

    // 清理并渲染
    ctx.clearRect(0, 0, canvasSize, canvasSize)
    draw()

    requestAnimationFrame(render)
  }, 100)
}

// 开始游戏
// 创建游戏对象，计算画布边长
// 显示计分板，重置画布宽高
// 调用 start 方法，改变游戏状态
// 开始循环渲染
async function run() {
  game = Game.new(cellSize, cellCount, defaultLength, defaultDirection, target)
  canvasSize = game.canvas_size()

  scoreContainer.style.display = ''

  canvas.width = canvasSize
  canvas.height = canvasSize

  game.start()

  requestAnimationFrame(render)
}

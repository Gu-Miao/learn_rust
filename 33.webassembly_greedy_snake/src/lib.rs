use rand::Rng;
// wasm_bindgen 的预导入模块
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// 将 wee_alloc 注册为全局内存分配器
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// 创建随机数
fn random(max: usize) -> usize {
  rand::thread_rng().gen_range(0..max)
}

// 坐标，对应画布上的 x, y 坐标
#[wasm_bindgen]
pub struct Coordinate(pub usize, pub usize);

#[wasm_bindgen]
impl Coordinate {
  // 新建方法，主要提供给前端调用
  pub fn new(x: usize, y: usize) -> Self {
    Self(x, y)
  }
  // 转换为索引
  // 横坐标 + 纵坐标 * 单向画布格子数
  pub fn to_index(&self, cell_count: usize) -> Index {
    Index::new(self.0 + self.1 * cell_count)
  }
}

// 索引
// 因为蛇身使用 Vector 保存，所以需要实现 Clone 和 Copy trait
// 再实现 PartialEq 方便索引进行判等
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct Index(pub usize);

#[wasm_bindgen]
impl Index {
  // 新建索引
  pub fn new(index: usize) -> Self {
    Self(index)
  }
  // 转换为坐标
  // 横坐标 = 索引 % 单向画布格子数
  // 纵坐标 = 索引 / 单向画布格子数
  pub fn to_coordinate(&self, cell_count: usize) -> Coordinate {
    Coordinate::new(self.0 % cell_count, self.0 / cell_count)
  }
}

// 方向
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

// 蛇
#[wasm_bindgen]
pub struct Snake {
  body_indices: Vec<Index>, // 蛇身
  direction: Direction,     // 当前蛇的方向
}

#[wasm_bindgen]
impl Snake {
  // 创建蛇
  // heading_index 蛇头坐标
  // len           蛇身长度
  // cell_count    单向画布格子数
  // direction     方向
  pub fn new(heading_index: usize, len: usize, cell_count: usize, direction: Direction) -> Self {
    let mut body_indices = Vec::new();

    // 根据方向和长度生成蛇身
    // 左上角为坐标原点
    for i in 0..len {
      match direction {
        Direction::Left => body_indices.push(Index::new(heading_index + i)),
        Direction::Right => body_indices.push(Index::new(heading_index - i)),
        Direction::Up => body_indices.push(Index::new(heading_index + i * cell_count)),
        Direction::Down => body_indices.push(Index::new(heading_index - i * cell_count)),
      };
    }

    Self {
      body_indices,
      direction,
    }
  }
}

// 游戏状态
// 分为闲置（未开始），游戏中，胜利和失败四种
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum Status {
  Idle,
  Playing,
  Won,
  Lost,
}

// 游戏对象
#[wasm_bindgen]
pub struct Game {
  cell_size: usize,    // 画布格子的大小
  cell_count: usize,   // 单向画布格子数量
  reward_index: Index, // 蛋的索引（位置）
  snake: Snake,        // 蛇
  status: Status,      // 游戏状态
  target: usize,       // 目标分数，达成即可胜利
  score: usize,        // 当前分数
}

#[wasm_bindgen]
impl Game {
  // 创建新游戏
  pub fn new(
    cell_size: usize,
    cell_count: usize,
    default_snake_len: usize,
    direction: Direction,
    target: usize,
  ) -> Self {
    // 使用 random 函数创建随机蛇头位置
    let snake = Snake::new(
      random(cell_count.pow(2)),
      default_snake_len,
      cell_count,
      direction,
    );

    Self {
      cell_size,
      cell_count,
      reward_index: Game::gen_reward_index(cell_count, &snake.body_indices),
      snake,
      status: Status::Idle,
      target,
      score: 0,
    }
  }

  // 生成蛋的索引（位置）
  // 需要保证生成的蛋不在蛇身上
  // loop 块是一个表达式，break 后跟的就是表达式的值
  fn gen_reward_index(cell_count: usize, vec: &Vec<Index>) -> Index {
    loop {
      let index = Index::new(random(cell_count.pow(2)));
      if !vec.contains(&index) {
        break index;
      }
    }
  }

  // 获取当前分数
  pub fn score(&self) -> usize {
    self.score
  }

  // 获取当前状态
  pub fn status(&self) -> Status {
    self.status
  }

  // 开始游戏，将状态改为游戏中
  pub fn start(&mut self) {
    self.status = Status::Playing;
  }

  // 获取蛋的索引（位置）
  pub fn reward_index(&self) -> Index {
    self.reward_index
  }

  // 获取画布边长
  pub fn canvas_size(&self) -> usize {
    self.cell_size * self.cell_count
  }

  // 获取蛇身索引
  // 这里需要返回一个裸指针，前端通过 Unit32Array 接收
  // 详见 www/index.ts 中的 drawSnake 函数
  pub fn snake_body_indices(&self) -> *const Index {
    self.snake.body_indices.as_ptr()
  }

  // 获取蛇身长度
  pub fn snake_len(&self) -> usize {
    self.snake.body_indices.len()
  }

  // 转向
  // 不能让蛇直接回头
  pub fn turn(&mut self, direction: Direction) {
    let current = &self.snake.direction;
    if (current == &Direction::Up && direction == Direction::Down)
      | (current == &Direction::Down && direction == Direction::Up)
      | (current == &Direction::Left && direction == Direction::Right)
      | (current == &Direction::Right && direction == Direction::Left)
    {
      return;
    }

    self.snake.direction = direction;
  }

  // 计算下一次渲染蛇头的位置
  // 左：x - 1
  // 右：x + 1
  // 上：x - cell_count
  // 下：y + cell_count
  // 再对 cell_count 取余使它们可以穿过边界，最后转为索引返回
  fn next_heading_index(&self, direction: &Direction) -> Index {
    let body_indeices = self.snake.body_indices.clone();
    let Coordinate(mut x, mut y) = body_indeices[0].to_coordinate(self.cell_count);

    let coordinate = match direction {
      Direction::Left => {
        if x == 0 {
          x = self.cell_count;
        }
        Coordinate::new((x - 1) % self.cell_count, y)
      }
      Direction::Right => Coordinate::new((x + 1) % self.cell_count, y),
      Direction::Up => {
        if y == 0 {
          y = self.cell_count;
        }
        Coordinate::new(x, (y - 1) % self.cell_count)
      }
      Direction::Down => Coordinate::new(x, (y + 1) % self.cell_count),
    };

    coordinate.to_index(self.cell_count)
  }

  // 更新
  // 此方法只有状态为游戏中时才会执行
  // 首先计算出下一次渲染时蛇头的位置，如果该位置与蛇身重叠，游戏失败
  // 否则将其改为蛇身的第一个元素，然后将蛇身中的元素都后移一位，以此达到蛇头带动蛇身的效果
  //
  // [0] [1] [2] [3] [4] [5] -> [new] [0] [1] [2] [3] [4]
  //
  // 如果蛇头碰到了蛋，那么将蛋的索引推入蛇身的最后一个位置，此时蛋和蛇身重叠，但下一帧，“蛋”
  // 就会变成上一帧最后一个元素，蛇身就被拉长了。
  // 再重新生成一个蛋并增加积分
  //
  // 如果目标分数达成，游戏胜利
  pub fn update(&mut self) {
    if self.status != Status::Playing {
      return;
    }

    let clone = self.snake.body_indices.clone();
    let next_heading_index = self.next_heading_index(&self.snake.direction);

    if clone.contains(&next_heading_index) {
      self.status = Status::Lost;
      return;
    }

    self.snake.body_indices[0] = next_heading_index;

    for i in 1..self.snake_len() {
      self.snake.body_indices[i] = clone[i - 1]
    }

    if self.reward_index == self.snake.body_indices[0] {
      self.snake.body_indices.push(self.reward_index);
      self.reward_index = Self::gen_reward_index(self.cell_count, &self.snake.body_indices);
      self.score += 1;
    }

    if self.score == self.target {
      self.status = Status::Won;
    }
  }
}

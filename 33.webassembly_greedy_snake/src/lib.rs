use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct CanvasData {
  cell_size: usize,
  cell_count: usize,
  snake: Snake,
}

#[wasm_bindgen]
impl CanvasData {
  pub fn new(cell_size: usize, cell_count: usize, heading_index: usize) -> Self {
    Self {
      cell_size,
      cell_count,
      snake: Snake::new(heading_index),
    }
  }

  pub fn canvas_size(&self) -> usize {
    self.cell_size * self.cell_count
  }

  pub fn heading_index(&self) -> usize {
    self.snake.body[0]
  }

  fn set_heading_index(&mut self, index: usize) {
    self.snake.body[0] = index;
  }

  pub fn turn(&mut self, direction: Direction) {
    self.snake.direction = direction;
  }

  pub fn update(&mut self) {
    let (mut x, mut y) = self.index_to_coordinate(self.snake.body[0]);
    let (x, y) = match self.snake.direction {
      Direction::Left => {
        if x == 0 {
          x = self.cell_count;
        }
        ((x - 1) % self.cell_count, y)
      }
      Direction::Right => ((x + 1) % self.cell_count, y),
      Direction::Up => {
        if y == 0 {
          y = self.cell_count;
        }
        (x, (y - 1) % self.cell_count)
      }
      Direction::Down => (x, (y + 1) % self.cell_count),
    };
    self.set_heading_index(self.coordinate_to_index(x, y));
  }

  fn index_to_coordinate(&self, index: usize) -> (usize, usize) {
    (index % self.cell_count, index / self.cell_count)
  }

  fn coordinate_to_index(&self, x: usize, y: usize) -> usize {
    x + y * self.cell_count
  }
}

#[wasm_bindgen]
pub enum Direction {
  Left,
  Right,
  Up,
  Down,
}

#[wasm_bindgen]
pub struct Snake {
  body: Vec<usize>,
  direction: Direction,
}

#[wasm_bindgen]
impl Snake {
  pub fn new(heading_index: usize) -> Self {
    Self {
      body: vec![heading_index],
      direction: Direction::Right,
    }
  }
}

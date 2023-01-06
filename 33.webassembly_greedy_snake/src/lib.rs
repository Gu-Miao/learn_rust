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

  pub fn update(&mut self) {
    self.snake.body[0] = if self.snake.body[0] + 1 > self.canvas_size() {
      0
    } else {
      self.snake.body[0] + 1
    }
  }
}

#[wasm_bindgen]
pub struct Snake {
  body: Vec<usize>,
}

#[wasm_bindgen]
impl Snake {
  pub fn new(heading_index: usize) -> Self {
    Self {
      body: vec![heading_index],
    }
  }
}

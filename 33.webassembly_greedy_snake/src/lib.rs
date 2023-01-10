use rand::Rng;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

fn random(max: usize) -> usize {
  rand::thread_rng().gen_range(0..max)
}

#[wasm_bindgen]
pub struct Coordinate(pub usize, pub usize);

#[wasm_bindgen]
impl Coordinate {
  pub fn new(x: usize, y: usize) -> Self {
    Self(x, y)
  }
  pub fn to_index(&self, cell_count: usize) -> Index {
    Index::new(self.0 + self.1 * cell_count)
  }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct Index(pub usize);

#[wasm_bindgen]
impl Index {
  pub fn new(index: usize) -> Self {
    Self(index)
  }
  pub fn to_coordinate(&self, cell_count: usize) -> Coordinate {
    Coordinate::new(self.0 % cell_count, self.0 / cell_count)
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
  body_indices: Vec<Index>,
  direction: Direction,
}

#[wasm_bindgen]
impl Snake {
  pub fn new(heading_index: usize, len: usize, cell_count: usize, direction: Direction) -> Self {
    let mut body_indices = Vec::new();

    for i in 0..len {
      match direction {
        Direction::Left => body_indices.push(Index::new(heading_index - i)),
        Direction::Right => body_indices.push(Index::new(heading_index + i)),
        Direction::Up => body_indices.push(Index::new(heading_index - i * cell_count)),
        Direction::Down => body_indices.push(Index::new(heading_index + i * cell_count)),
      };
    }

    Self {
      body_indices,
      direction,
    }
  }
}

#[wasm_bindgen]
pub struct CanvasData {
  cell_size: usize,
  cell_count: usize,
  reward_index: Index,
  snake: Snake,
}

#[wasm_bindgen]
impl CanvasData {
  pub fn new(cell_size: usize, cell_count: usize, len: usize, direction: Direction) -> Self {
    let snake = Snake::new(random(cell_count.pow(2)), len, cell_count, direction);

    Self {
      cell_size,
      cell_count,
      reward_index: CanvasData::gen_reward_index(cell_count.pow(2), &snake.body_indices),
      snake,
    }
  }

  fn gen_reward_index(max: usize, vec: &Vec<Index>) -> Index {
    loop {
      let index = Index::new(random(max));
      if !vec.contains(&index) {
        break index;
      }
    }
  }

  pub fn reward_index(&self) -> usize {
    self.reward_index.0
  }

  pub fn canvas_size(&self) -> usize {
    self.cell_size * self.cell_count
  }

  pub fn snake_body_indices(&self) -> *const Index {
    self.snake.body_indices.as_ptr()
  }

  pub fn snake_len(&self) -> usize {
    self.snake.body_indices.len()
  }

  pub fn turn(&mut self, direction: Direction) {
    let next_heading_index = self.next_heading_index(&direction);
    if next_heading_index != self.snake.body_indices[1] {
      self.snake.direction = direction;
    }
  }

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

  pub fn update(&mut self) {
    let clone = self.snake.body_indices.clone();
    self.snake.body_indices[0] = self.next_heading_index(&self.snake.direction);

    for i in 1..self.snake_len() {
      self.snake.body_indices[i] = clone[i - 1]
    }
  }
}

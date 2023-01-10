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
  pub fn from_index(Index(index): Index, cell_count: usize) -> Self {
    Self::new(index % cell_count, index / cell_count)
  }
  pub fn to_index(&self, cell_count: usize) -> Index {
    Index::new(self.0 + self.1 * cell_count)
  }
}

#[wasm_bindgen]
pub struct Index(pub usize);

#[wasm_bindgen]
impl Index {
  pub fn new(index: usize) -> Self {
    Self(index)
  }
  pub fn from_coordinate(Coordinate(x, y): Coordinate, cell_count: usize) -> Self {
    Self::new(x + y * cell_count)
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
      }
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
  reward_index: usize,
  snake: Snake,
}

#[wasm_bindgen]
impl CanvasData {
  pub fn new(cell_size: usize, cell_count: usize, len: usize, direction: Direction) -> Self {
    Self {
      cell_size,
      cell_count,
      reward_index: random(cell_count.pow(2)),
      snake: Snake::new(random(cell_count.pow(2)), len, cell_count, direction),
    }
  }

  pub fn reward_index(&self) -> usize {
    self.reward_index
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
    self.snake.direction = direction;
  }

  pub fn update(&mut self) {
    let Coordinate(mut x, mut y) = self.snake.body_indices[0].to_coordinate(self.cell_count);
    let coordinate = match self.snake.direction {
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

    self.snake.body_indices[0] = coordinate.to_index(self.cell_count);
  }
}

const MAP_SIZE: usize = 1500;
const MAP_ARRAY_SIZE: usize = MAP_SIZE * MAP_SIZE;

const MAP_DATA_INVALID: i32 = -1;
const _MAP_DATA_CLEANED: i32 = 11110;
const _MAP_DATA_FILLED: i32 = 11111;
const _MAP_DATA_THICKENED: i32 = 11113;
const _MAP_DATA_AVOID: i32 = 11115;

pub struct Map {
    data: Vec<i32>,
    max_size_x: i32,
    max_size_y: i32,
}

impl Map {
    pub fn new() -> Self {
        Self {
            data: vec![MAP_DATA_INVALID; MAP_ARRAY_SIZE],
            max_size_x: 0,
            max_size_y: 0,
        }
    }

    pub fn reset(&mut self) {
        self.max_size_x = 0;
        self.max_size_y = 0;
        self.data.fill(MAP_DATA_INVALID);
    }

    fn offset(&self, x: i32, y: i32) -> usize {
        (x as usize * MAP_SIZE + y as usize).min(MAP_ARRAY_SIZE - 1)
    }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        self.data[self.offset(x, y)]
    }

    pub fn set(&mut self, x: i32, y: i32, value: i32) {
        if value % 2 == 0 {
            if x > self.max_size_x {
                self.max_size_x = x;
            }
            if y > self.max_size_y {
                self.max_size_y = y;
            }
        }
        let offset = self.offset(x, y);
        self.data[offset] = value;
    }

    #[allow(dead_code)]
    pub fn max_x(&self) -> i32 {
        self.max_size_x
    }

    pub fn max_y(&self) -> i32 {
        self.max_size_y
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

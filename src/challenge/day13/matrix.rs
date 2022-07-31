#[derive(Debug)]
pub struct Matrix {
    inner: Vec<i32>,
    size: usize,
}

impl Matrix {
    pub fn new(size: usize) -> Matrix {
        Matrix {
            inner: vec![0; size * size],
            size,
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        self.size * y + x
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        let idx = self.idx(x, y);
        self.inner[idx]
    }

    pub fn set(&mut self, x: usize, y: usize, value: i32) {
        let idx = self.idx(x, y);
        self.inner[idx] = value
    }
}

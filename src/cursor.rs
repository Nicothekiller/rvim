#[derive(Debug)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x_copy(&mut self) -> usize {
        self.x
    }

    pub fn y_copy(&mut self) -> usize {
        self.y
    }

    pub fn x_mut(&mut self) -> &mut usize {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut usize {
        &mut self.y
    }
}

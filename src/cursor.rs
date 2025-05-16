/// Handles the Cursor. Its just 2 usize, one for x and one for y.
#[derive(Debug)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    /// Creates a new [`Cursor`].
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Returns x as a copy.
    pub fn x_copy(&mut self) -> usize {
        self.x
    }

    /// Returns y as a copy.
    pub fn y_copy(&mut self) -> usize {
        self.y
    }

    /// Returns a mutable reference to the x of this [`Cursor`].
    pub fn x_mut(&mut self) -> &mut usize {
        &mut self.x
    }

    /// Returns a mutable reference to the y of this [`Cursor`].
    pub fn y_mut(&mut self) -> &mut usize {
        &mut self.y
    }
}

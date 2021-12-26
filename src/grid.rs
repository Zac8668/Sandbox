pub struct Grid {
    pub vec: Vec<Vec<bool>>,
    pub width: u32,
    pub height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let vec = vec![vec![false; width as usize]; height as usize];

        Self { vec, width, height }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<u8>,
}

pub fn generate(width: usize, height: usize) -> Grid {
    Grid {
        width,
        height,
        cells: vec![1; width * height],
    }
}

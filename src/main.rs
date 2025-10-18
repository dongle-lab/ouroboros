mod maze;

use crate::maze::*;

fn main() {
    let g: Grid = generate(20, 10);
    println!("{}x{} maze with {} cells", g.width, g.height, g.cells.len());
}

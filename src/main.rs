mod grid;
use grid::Grid;

fn main() {
    println!("Hello, world!");
    let mut grid = Grid::new((10, 10));
    grid.init(10);
    grid.print();
}

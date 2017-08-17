extern crate mazes;
use mazes::*;

fn main() {
    let mut maze = Maze::new(2, 2);
    let p1 = Point::new(0, 0);
    let p2 = Point::new(1, 0);
    maze.add_neighbor(p1, p2);
    println!("maze: {:?}", maze);
}

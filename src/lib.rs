use std::collections::HashMap;
use std::cmp::Ordering::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    column: u32,
    row: u32,
}

impl Point {
    pub fn new(column: u32, row: u32) -> Point {
        Point {
            column,
            row,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cell {
    loc: Point,
    neighbors: HashMap<Direction, Point>,
}

impl Cell {
    pub fn new(loc: Point) -> Cell {
        Cell {
            loc,
            neighbors: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    cells: HashMap<Point, Cell>,
}

impl Maze {
    pub fn new(x: u32, y: u32) -> Maze {
        let mut cells = HashMap::new();
        for column in 0..x {
            for row in 0..y {
                let p = Point {
                    column,
                    row
                };
                let mut c = Cell::new(p);
                cells.insert(p, c);
            }
        }
        Maze {
            cells,
        }
    }

    fn calc_dir(p1: Point, p2: Point) -> Direction {
        let col_diff = p1.column.cmp(&p2.column);
        let row_diff = p1.row.cmp(&p2.row);
        match (col_diff, row_diff) {
            (Less, Equal) => Direction::East,
            (Greater, Equal) => Direction::West,
            (Equal, Greater) => Direction::North,
            (Equal, Less) => Direction::South,
            _ => panic!("Invalid cell pair")
        }
    }

    pub fn add_neighbor(&mut self, p1: Point, p2: Point) {
        let d1 = Maze::calc_dir(p1, p2);
        let d2 = Maze::calc_dir(p2, p1);
        self.cells.get_mut(&p1).map(|c| c.neighbors.insert(d1, p2));
        self.cells.get_mut(&p2).map(|c| c.neighbors.insert(d2, p1));
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::Direction::*;

    #[test]
    fn calc_dir() {
        let center = Point {
            column: 1,
            row: 1,
        };
        let n = Point {
            column: 1,
            row: 0,
        };
        let s = Point {
            column: 1,
            row: 2,
        };
        let e = Point {
            column: 2,
            row: 1,
        };
        let w = Point {
            column: 0,
            row: 1,
        };
        assert_eq!(North, Maze::calc_dir(center, n));
        assert_eq!(South, Maze::calc_dir(center, s));
        assert_eq!(East, Maze::calc_dir(center, e));
        assert_eq!(West, Maze::calc_dir(center, w));
    }

    #[test]
    #[should_panic]
    fn calc_dir_bad() {
        let center = Point {
            column: 0,
            row: 0,
        };
        let bad = Point {
            column: 1,
            row: 1,
        };
        let _ = Maze::calc_dir(center, center);
        let _ = Maze::calc_dir(center, bad);
    }

    #[test]
    fn add_neighbor() {
        let mut maze = Maze::new(2, 2);
        let p1 = Point::new(0, 0); 
        let p2 = Point::new(1, 0);
        maze.add_neighbor(p1, p2);
        let c1 = maze.cells.get(&p1).unwrap();
        let c2 = maze.cells.get(&p2).unwrap();
        assert_eq!(&p2, c1.neighbors.get(&East).expect("p2 not found"));
        assert_eq!(&p1, c2.neighbors.get(&West).expect("p1 not found"));
    }
}

extern crate rand;
use std::collections::HashMap;
use std::cmp::Ordering::*;
use rand::distributions::{IndependentSample, Range};

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

struct EachRow {
    maze: &Maze,
    ctr: u32,
}

impl EachRow {
    fn new(maze: &Maze) -> EachRow {
        EachRow {
            ctr: 0,
            maze,
        }
    }
    fn inc_ctr(&mut self) {
        self.ctr += 1;
    }
}

impl Iterator for EachRow {
    type Item = Vec<&Cell>;

    fn next(&self) -> Option<Vec<&Cell>> {
        if self.ctr > self.maze.row_count {
            None
        } else {
            let &mut v = Vector::new();
            for col_idx in 0..self.maze.col_count {
                let p = Point::new(self.ctr, col_idx);
                v.push(self.maze.cells[&p]);
            }
            self.inc_ctr();
            Some(v)
        }
    }
}

impl EachCol {
    fn new(maze: &Maze) -> EachCol {
        EachCol {
            ctr: 0,
            maze,
        }
    }
    fn inc_ctr(&mut self) {
        self.ctr += 1;
    }
}

impl Iterator for EachCol {
    type Item = Vec<&Cell>;
    fn next(&self) -> Option<Vec<&Cell>> {
        if self.ctr > self.maze.col_count {
            None
        } else {
            let &mut v = Vector::new();
            for row_idx in 0..self.maze.row_count {
                let p = Point::new(self.ctr, row_idx);
                v.push(self.maze.cells[&p]);
            }
            self.inc_ctr();
            Some(v)
        }
    }
}

struct EachCell {
    maze: &maze,
    col_ctr: u32,
    row_ctr: u32,
}

impl EachCell {
    fn new(&maze) -> EachCell {
        EachCell {
            col_ctr: 0,
            row_ctr: 0,
            maze
        }
    }
    fn inc_ctrs(&mut self) {
        let new_col = self.col_ctr + 1;
        if new_col > self.maze.col_count {
            self.col_ctr = 0;
            self.row_ctr = self.row_ctr + 1;
        } else {
            self.col_ctr = new_col;
        }
    }
}

impl Iterator for EachCell {
    type Item = Cell;

    fn next(&self) -> Option<Cell> {
        if self.col_ctr == 0 && self.row_ctr > self.maze.row_count {
            None
        } else {
            let p = Point::new(self.row_ctr, self.col_ctr);
            let c = self.maze.cells[&p];
            self.inc_ctrs();
            Some(v)
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    row_count: u32,
    col_count: u32,
    cells: HashMap<Point, Cell>,
}

impl Maze {
    pub fn new(col_count: u32, row_count: u32) -> Maze {
        let mut cells = HashMap::new();
        for column in 0..col_count {
            for row in 0..row_count {
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
            row_count,
            col_count,
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

    pub fn each_row_iter(&self) -> EachRow {
        EachRow::new(self.maze)
    }

    pub fn each_col_iter(&self) -> EachCol {
        EachCol::new(self.maze)
    }
    
    pub fn each_cell_iter(&self) -> EachCell {
        EachCell::new(self.maze)
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

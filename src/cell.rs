use std::collections::HashMap;
use std::cmp::Ordering::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub column: u32,
    pub row: u32,
}

impl Point {
    pub fn new(column: u32, row: u32) -> Point {
        Point {
            column,
            row,
        }
    }
    pub fn calc_dir(p1: Point, p2: Point) -> Direction {
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cell {
    pub loc: Point,
    pub neighbors: HashMap<Direction, Point>,
}

impl Cell {
    pub fn new(loc: Point) -> Cell {
        Cell {
            loc,
            neighbors: HashMap::new(),
        }
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
        assert_eq!(North, Point::calc_dir(center, n));
        assert_eq!(South, Point::calc_dir(center, s));
        assert_eq!(East, Point::calc_dir(center, e));
        assert_eq!(West, Point::calc_dir(center, w));
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
        let _ = Point::calc_dir(center, center);
        let _ = Point::calc_dir(center, bad);
    }
}

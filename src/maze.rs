use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, ThreadRng};

use cell::{Direction, Point, Cell};

pub struct EachRow<'a> {
    maze: &'a Maze,
    ctr: u32,
}

impl<'a> EachRow<'a> {
    fn new(maze: &'a Maze) -> EachRow {
        EachRow {
            ctr: 0,
            maze,
        }
    }
    fn inc_ctr(&mut self) {
        self.ctr += 1;
    }
}

impl<'a> Iterator for EachRow<'a> {
    type Item = Vec<&'a Cell>;

    fn next(&mut self) -> Option<Vec<&'a Cell>> {
        if self.ctr >= self.maze.row_count {
            None
        } else {
            let mut v = Vec::new();
            for col_idx in 0..self.maze.col_count {
                let p = Point::new(self.ctr, col_idx);
                v.push(&self.maze.cells[&p]);
            }
            self.inc_ctr();
            Some(v)
        }
    }
}

pub struct EachCol<'a> {
    maze: &'a Maze,
    ctr: u32,
}

impl<'a> EachCol<'a> {
    fn new(maze: &'a Maze) -> EachCol {
        EachCol {
            ctr: 0,
            maze,
        }
    }
    fn inc_ctr(&mut self) {
        self.ctr += 1;
    }
}

impl<'a> Iterator for EachCol<'a> {
    type Item = Vec<&'a Cell>;

    fn next(&mut self) -> Option<Vec<&'a Cell>> {
        if self.ctr >= self.maze.col_count {
            None
        } else {
            let mut v = Vec::new();
            for row_idx in 0..self.maze.row_count {
                let p = Point::new(row_idx, self.ctr);
                v.push(&self.maze.cells[&p]);
            }
            self.inc_ctr();
            Some(v)
        }
    }
}

pub struct EachCell<'a> {
    maze: &'a Maze,
    col_ctr: u32,
    row_ctr: u32,
}

impl<'a> EachCell<'a> {
    fn new(maze: &'a Maze) -> EachCell {
        EachCell {
            col_ctr: 0,
            row_ctr: 0,
            maze
        }
    }
    fn inc_ctrs(&mut self) {
        let new_col = self.col_ctr + 1;
        if new_col >= self.maze.col_count {
            self.col_ctr = 0;
            self.row_ctr = self.row_ctr + 1;
        } else {
            self.col_ctr = new_col;
        }
    }
}

impl<'a> Iterator for EachCell<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        if self.col_ctr == 0 && self.row_ctr >= self.maze.row_count {
            None
        } else {
            let p = Point::new(self.row_ctr, self.col_ctr);
            let ref c = self.maze.cells[&p];
            self.inc_ctrs();
            Some(c)
        }
    }
}

pub struct RandCell<'a> {
    maze: &'a Maze,
    rng: ThreadRng,
    col_gen: Range<u32>,
    row_gen: Range<u32>,
    seen_count: u32,
}

impl<'a> RandCell<'a> {
    pub fn new(maze: &'a Maze) -> RandCell {
        let rng = thread_rng();
        let col_gen = Range::new(0, maze.col_count);
        let row_gen = Range::new(0, maze.row_count);
        RandCell {
            maze,
            rng,
            col_gen,
            row_gen,
            seen_count: 0,
        }
    }

    fn rand_point(&mut self) -> Point {
        Point::new(self.col_gen.ind_sample(&mut self.rng),
                   self.row_gen.ind_sample(&mut self.rng))
    }
}

impl<'a> Iterator for RandCell<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        if self.seen_count < self.maze.row_count * self.maze.col_count {
            let p = self.rand_point();
            let ref c = self.maze.cells[&p];
            self.seen_count += 1;
            Some(c)
        } else {
            None
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
                let p = Point::new(column, row);
                let c = Cell::new(p);
                cells.insert(p, c);
            }
        }
        Maze {
            cells,
            row_count,
            col_count,
        }
    }

    pub fn get_neighbor(&self, p1: Point, dir: Direction) -> &Cell {
        &self.cells[&p1.get_neighbor(dir)]
    }

    pub fn add_neighbor(&mut self, p1: Point, p2: Point) {
        let d1 = Point::calc_dir(p1, p2);
        let d2 = Point::calc_dir(p2, p1);
        self.cells.get_mut(&p1).map(|c| c.neighbors.insert(d1, p2));
        self.cells.get_mut(&p2).map(|c| c.neighbors.insert(d2, p1));
    }

    pub fn each_row_iter(&self) -> EachRow {
        EachRow::new(self)
    }

    pub fn each_col_iter(&self) -> EachCol {
        EachCol::new(self)
    }

    pub fn each_cell_iter(&self) -> EachCell {
        EachCell::new(self)
    }

    pub fn rand_cell_iter(&self) -> RandCell {
        RandCell::new(self)
    }
}

impl Index<Point> for Maze {
    type Output = Cell;

    fn index<'a>(&'a self, p: Point) -> &'a Cell {
        &self.cells[&p]
    }
}

impl IndexMut<Point> for Maze {
    fn index_mut<'a>(&'a mut self, p: Point) -> &'a mut Cell {
        self.cells.get_mut(&p).unwrap()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use cell::Direction::*;

    fn new_maze() -> Maze {
        Maze::new(2,2)
    }

    #[test]
    fn add_neighbor() {
        let mut maze = new_maze();
        let p1 = Point::new(0, 0); 
        let p2 = Point::new(1, 0);
        maze.add_neighbor(p1, p2);
        let c1 = maze.cells.get(&p1).unwrap();
        let c2 = maze.cells.get(&p2).unwrap();
        assert_eq!(&p2, c1.neighbors.get(&East).expect("p2 not found"));
        assert_eq!(&p1, c2.neighbors.get(&West).expect("p1 not found"));
    }

    #[test]
    fn each_row_iter() {
        let maze = new_maze();
        let mut eri = maze.each_row_iter();
        let r0 = eri.next().expect("row 0 missing");
        let ref r0c0 = maze.cells[&Point::new(0, 0)];
        let ref r0c1 = maze.cells[&Point::new(0, 1)];
        assert_eq!(vec![r0c0, r0c1], r0);
        let r1 = eri.next().expect("row 1 missing");
        let ref r1c0 = maze.cells[&Point::new(1, 0)];
        let ref r1c1 = maze.cells[&Point::new(1, 1)];
        assert_eq!(vec![r1c0, r1c1], r1);
        if let Some(_) = eri.next() {
            panic!("extra row");
        }
    }

    #[test]
    fn each_col_iter() {
        let maze = new_maze();
        let mut eci = maze.each_col_iter();
        let c0 = eci.next().expect("col 0 missing");
        let ref r0c0 = maze.cells[&Point::new(0, 0)];
        let ref r1c0 = maze.cells[&Point::new(1, 0)];
        assert_eq!(vec![r0c0, r1c0], c0);
        let c1 = eci.next().expect("col 1 missing");
        let ref r0c1 = maze.cells[&Point::new(0, 1)];
        let ref r1c1 = maze.cells[&Point::new(1, 1)];
        assert_eq!(vec![r0c1, r1c1], c1);
        if let Some(_) = eci.next() {
            panic!("extra row");
        }
    }

    #[test]
    fn each_cell_iter() { 
        let maze = new_maze();
        let ref r0c0 = maze.cells[&Point::new(0, 0)];
        let ref r0c1 = maze.cells[&Point::new(0, 1)];
        let ref r1c0 = maze.cells[&Point::new(1, 0)];
        let ref r1c1 = maze.cells[&Point::new(1, 1)];
        let expected_cells = vec![r0c0, r0c1, r1c0, r1c1];
        let actual_cells: Vec<&Cell> = maze.each_cell_iter().collect();
        assert_eq!(expected_cells, actual_cells);
    }

    #[test]
    fn rand_cell_iter() {
        let maze = new_maze();
        let mut cnt = 0;
        for c in maze.rand_cell_iter() {
            assert!(c.loc.column < maze.col_count);
            assert!(c.loc.column >= 0);
            assert!(c.loc.row < maze.row_count);
            assert!(c.loc.column >= 0);
            cnt += 1;
        }
        assert_eq!(4, cnt);
    }
}

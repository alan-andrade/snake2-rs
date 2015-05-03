// From top left moving right and down.
//
// (x, y)
//
// 1,1 ------ 2,1
//  |          |
//  |          |
//  |          |
// 1,2 ------ 2,2
//

#[derive(Eq, PartialEq, Ord, Clone, Copy)]
pub struct Position(pub u8, pub u8);

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let &Position(x,y) = self;
        write!(f, "[{}, {}]\t", x, y)
    }
}

use std::fmt::{Formatter, Display, Error};

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Position(x,y) = *self;
        try!(write!(f, "({:?}, {:?})", x, y));
        Display::fmt("",f)
    }
}

use std::cmp::Ordering;

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        let Position(x1, y1) = *self;
        let Position(x2, y2) = *other;

        if x1 == x2 && y1 == y2 {
            return Some(Ordering::Equal)
        } else if (y1 > y2) && (x1 > x2) {
            return Some(Ordering::Greater)
        } else {
            return Some(Ordering::Less)
        }

        None
    }
}

#[test]
fn position_is_ordinal() {
    let left_up = Position(1, 1);
    let right_down = Position(2, 2);

    assert!(left_up < right_down);
    assert!(left_up == left_up);
    assert!(right_down > left_up);
}

use std::collections::BTreeMap;
use std::fmt::Debug;

pub struct Grid {
    source: BTreeMap<Position, Object>,
    pub width: u8,
    pub height: u8
}

impl Debug for Grid {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(writeln!(fmt, "Grid:"));
        for row in ByRow::new(&self) {
            for position in row {
                if let Some(value) = self.source.get(&position) {
                    try!(write!(fmt, "{:?}", value));
                }
            }
            try!(writeln!(fmt, "\n"));
        }
        write!(fmt, "--")
    }
}

pub static MIN_WIDTH : u8 = 1;
pub static MIN_HEIGHT : u8 = 1;

fn in_bound(grid: &Grid, position: &Position) -> bool {
    let &Position(x, y) = position;

    if x > grid.width || y > grid.height ||
        x < MIN_WIDTH || y < MIN_HEIGHT {
        false
    } else {
        true
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Object {
    Snake,
    Apple,
    Empty,
    Wall
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match *self {
            Object::Snake => "*",
            Object::Empty => ".",
            Object::Wall => "#",
            Object::Apple => "O"
        })
    }
}

struct ByRow {
    generator: PositionGenerator
}

impl ByRow {
    fn new(grid: &Grid) -> ByRow {
        ByRow {
            generator: PositionGenerator::new(grid.width, grid.height)
        }
    }
}

impl Iterator for ByRow {
    type Item = Vec<Position>;

    fn next(&mut self) -> Option<Vec<Position>> {
        let mut collector : Vec<Position> = vec!();
        let (w, _) = self.generator.limits;

        loop {
            if let Some(Position(x, y))  = self.generator.next() {
                collector.push(Position(x,y));
                if x == w {
                    return Some(collector);
                }
            } else {
                return None;
            }
        }
    }
}

struct PositionGenerator {
    pub limits: (u8,u8),
    current: Position
}

impl PositionGenerator {
    fn new(w: u8, h: u8) -> PositionGenerator {
        PositionGenerator {
            limits:(w,h),
            current: Position(0, 1)
        }
    }
}

impl Iterator for PositionGenerator {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let Position(mut x, mut y) = self.current;
        let (w,h) = self.limits;

        if x == w {
            y = y + 1;
            x = 0
        }

        if x < w {
            x = x + 1;
        }

        if y > h {
            return None;
        }

        self.current = Position(x,y);
        return Some(self.current);
    }
}


impl Grid {
    pub fn new(w: u8, h: u8) -> Grid {
        let mut source = BTreeMap::new();

        for position in PositionGenerator::new(w, h) {
            source.insert(position, Object::Empty);
        }

        Grid {
            source: source,
            width: w,
            height: h
        }
    }

    pub fn center(&mut self) -> Position {
        return Position(self.width/2,  self.height/2);
    }

    pub fn allocate_object_at(&mut self, object: Object, position: Position) ->
        AllocationEvent<Object> {
        if !in_bound(&self, &position) {
            return AllocationEvent::OutOfBounds
        }

        if let Some(existent) = self.source.insert(position, object) {
            if existent == Object::Empty {
                 AllocationEvent::Allocated
            } else {
                AllocationEvent::Collition(existent)
            }
        } else {
            AllocationEvent::Allocated
        }
    }

    pub fn free(&mut self, position: &Position) {
        self.source.insert(*position, Object::Empty);
    }

    pub fn move_object(&mut self, from: Position, to: Position) -> AllocationEvent<Object> {
        if let Some(obj) = self.source.remove(&from) {
            self.allocate_object_at(obj, to)
        } else {
            AllocationEvent::EmptySpace
        }
    }

    pub fn object_at(&mut self, position: Position) -> Option<&Object> {
        if in_bound(&self, &position) {
            self.source.get(&position)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AllocationEvent<Object> {
    Allocated,
    Collition(Object),
    OutOfBounds,
    EmptySpace
}

#[test]
#[allow(unused_must_use)]
fn grid_collition() {
    let mut grid = Grid::new(3, 3);
    let foo = Object::Snake;
    let bar = Object::Empty;
    let position = Position(1, 1);

    grid.allocate_object_at(foo, position);

    match grid.allocate_object_at(bar, position) {
        AllocationEvent::Allocated => panic!(),
        AllocationEvent::Collition(e) => { assert_eq!(e, foo) }
        _ => { panic!() }
    }
}

#[test]
fn grid_allocate_object_at() {
    let mut grid = Grid::new(5, 5);
    let foo = Object::Snake;
    let position = Position(1, 1);

    match grid.allocate_object_at(foo, position) {
        AllocationEvent::Allocated => {
            match grid.object_at(position) {
                Some(object) => assert!(&foo == object),
                None => panic!()
            }
        },
        _ => { panic!() }
    }
}

#[test]
fn grid_allocs_out_of_bounds() {
    let mut grid = Grid::new(5, 5);
    let snake = Object::Snake;

    let position = Position(6, 6);
    assert_eq!(grid.allocate_object_at(snake, position), AllocationEvent::OutOfBounds);
    let position = Position(0, 0);
    assert_eq!(grid.allocate_object_at(snake, position), AllocationEvent::OutOfBounds);
}

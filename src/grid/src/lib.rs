// From top left moving right and down.
//
// (x, y)
//
// 0,0 ------ 1,0
//  |          |
//  |          |
//  |          |
// 0,1 ------ 1,1
//

#[derive(Debug, Eq, PartialEq, Ord, Clone, Copy)]
pub struct Position(pub u8, pub u8);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Snake,
    Apple,
    Wall
}

use std::collections::BTreeMap;

pub struct Grid {
    source: BTreeMap<Position, Object>,
    width: u8,
    height: u8
}

static MIN_WIDTH : u8 = 1;
static MIN_HEIGHT : u8 = 1;

fn in_bound(grid: &Grid, position: &Position) -> bool {
    let &Position(x, y) = position;

    if x > grid.width || y > grid.height ||
        x < MIN_WIDTH || y < MIN_HEIGHT {
        false
    } else {
        true
    }
}

impl Grid {
    pub fn new(w: u8, h: u8) -> Grid {
        Grid {
            source: BTreeMap::new(),
            width: w,
            height: h
        }
    }

    pub fn allocate_at(&mut self, position: Position, object: Object) -> AllocationEvent {
        if !in_bound(&self, &position) {
            return AllocationEvent::OutOfBounds
        }

        if let Some(existent) = self.source.insert(position, object) {
            AllocationEvent::Collition(existent)
        } else {
            AllocationEvent::Allocated
        }
    }

    pub fn move_object(&mut self, from: Position, to: Position) -> AllocationEvent {
        if let Some(obj) = self.source.remove(&from) {
            self.allocate_at(to, obj)
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
pub enum AllocationEvent {
    Allocated,
    Collition(Object),
    Yum,
    Crash,
    CollitionRuleMissing,
    OutOfBounds,
    EmptySpace
}

#[test]
#[allow(unused_must_use)]
fn grid_collition() {
    let mut grid = Grid::new(3, 3);
    let foo = Object::Snake;
    let bar = Object::Apple;
    let position = Position(1, 1);

    grid.allocate_at(position, foo);

    match grid.allocate_at(position, bar) {
        AllocationEvent::Allocated => panic!(),
        AllocationEvent::Collition(e) => { assert_eq!(e, foo) }
        _ => { panic!() }
    }
}

#[test]
fn grid_allocate_at() {
    let mut grid = Grid::new(5, 5);
    let foo = Object::Snake;
    let position = Position(1, 1);

    match grid.allocate_at(position, foo) {
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
fn grid_allocate_at_out_of_bounds() {
    let mut grid = Grid::new(5, 5);
    let snake = Object::Snake;

    let position = Position(6, 6);
    assert_eq!(grid.allocate_at(position, snake), AllocationEvent::OutOfBounds);
    let position = Position(0, 0);
    assert_eq!(grid.allocate_at(position, snake), AllocationEvent::OutOfBounds);
}

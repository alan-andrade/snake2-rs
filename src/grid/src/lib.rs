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

impl Grid {
    pub fn new(w: u8, h: u8) -> Grid {
        Grid {
            source: BTreeMap::new(),
            width: w,
            height: h
        }
    }

    pub fn allocate_at(&mut self, position: Position, object: Object) -> AllocationEvent {
        let Position(x, y) = position;

        if x > self.width || y > self.height {
            return AllocationEvent::OutOfBounds;
        }

        if let Some(existent) = self.source.insert(position, object) {
            return AllocationEvent::Collition(existent);
        } else {
            return AllocationEvent::Allocated
        }
    }

    fn object_at(&mut self, position: &Position) -> Option<&Object> {
        return self.source.get(position);
    }

    fn occupied_count(&self) -> u8 {
        return self.source.keys().count() as u8;
    }

    fn contains(&self, position: &Position) -> bool {
        return self.source.contains_key(position);
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AllocationEvent {
    Allocated,
    Collition(Object),
    Yum,
    Crash,
    CollitionRuleMissing,
    OutOfBounds
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
            assert!(grid.contains(&position));
            match grid.object_at(&position) {
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
}

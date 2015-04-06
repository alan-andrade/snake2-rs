extern crate rand;

use rand::distributions::{IndependentSample, Range};
use std::collections::BTreeMap;
use std::cmp::Ordering;

struct Grid {
    width: u8,
    height: u8,
    source: BTreeMap<Position, Object>
}

struct Object;

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

#[derive(Debug, Eq, PartialEq, Ord)]
struct Position(u8, u8);

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


impl Grid {
    fn new(w: u8, h: u8) -> Grid {
        Grid {
            source: BTreeMap::new(),
            width: w,
            height: h
        }
    }

    fn allocate(&mut self, object: Object) {
        // Extract random generator.
        let position = self.random_position();

        assert!(position < Position(self.width, self.height));
        self.source.insert(position, object);
    }

    fn occupied_count(&self) -> u8 {
        return self.source.keys().count() as u8;
    }

    fn contains(&self, position: &Position) -> bool {
        return self.source.contains_key(position);
    }

    fn random_position(&mut self) -> Position {
        let mut rng = rand::thread_rng();

        let x_range = Range::new(1, self.width);
        let y_range = Range::new(1, self.height);
        let mut position;

        loop {
            let x = x_range.ind_sample(&mut rng);
            let y = y_range.ind_sample(&mut rng);
            position = Position(x,y);
            if !self.contains(&position) { break; }
        }

        return position;
    }
}

#[test]
fn grid_allocate () {
    let mut grid = Grid::new(5, 5);

    let foo = Object;
    let bar = Object;

    grid.allocate(foo);
    grid.allocate(bar);

    assert_eq!(grid.occupied_count(), 2)
}

#[test]
fn ordinal() {
    let left_up = Position(1, 1);
    let right_down = Position(2, 2);

    assert!(left_up < right_down);
    assert!(left_up == left_up);
    assert!(right_down > left_up);
}

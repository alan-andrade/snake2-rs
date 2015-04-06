extern crate rand;

use rand::distributions::{IndependentSample, Range};
use rand::ThreadRng;
use std::collections::BTreeMap;
use std::cmp::Ordering;

struct Grid {
    width: u8,
    height: u8,
    source: BTreeMap<Position, Object>,
    generator: RandomGenerator
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Object {
    Foo,
    Bar
}

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

struct RandomGenerator {
    rng: ThreadRng,
    x_range: Range<u8>,
    y_range: Range<u8>
}

impl RandomGenerator {
    fn new(x_bound: u8, y_bound: u8) -> RandomGenerator {
        RandomGenerator {
            rng: rand::thread_rng(),
            x_range: Range::new(1, x_bound),
            y_range:  Range::new(1, y_bound)
        }
    }

    fn gen(&mut self) -> Position {
        return Position(self.x_range.ind_sample(&mut self.rng),
        self.y_range.ind_sample(&mut self.rng));
    }
}



impl Grid {
    fn new(w: u8, h: u8) -> Grid {
        Grid {
            source: BTreeMap::new(),
            width: w,
            height: h,
            generator: RandomGenerator::new(w, h)
        }
    }

    fn allocate(&mut self, object: Object) {
        let mut position;

        loop {
            // Still have to figure out what to do when no positions are left.
            position = self.generator.gen();
            if !self.contains(&position) { break; }
        }

        self.source.insert(position, object);
    }

    fn allocate_at(&mut self, position: Position, object: Object) {
        self.source.insert(position, object);
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

#[test]
fn grid_allocate() {
    let mut grid = Grid::new(5, 5);

    let foo = Object::Foo;
    let bar = Object::Bar;

    grid.allocate(foo);
    grid.allocate(bar);

    assert_eq!(grid.occupied_count(), 2)
}

fn grid_allocate_at() {
    let mut grid = Grid::new(5, 5);
    let foo = Object::Foo;
    let position = Position(1, 1);

    grid.allocate_at(position, foo);
    let position = Position(1, 1);

    assert!(grid.contains(&position));

    match grid.object_at(&position) {
        Some(object) => assert!(&foo == object),
        None => panic!()
    }
}

#[test]
fn random_generator() {
    let mut generator = RandomGenerator::new(5, 5);
    let x = generator.gen();
    let y = generator.gen();

    assert!(x != y);
}

#[test]
fn ordinal() {
    let left_up = Position(1, 1);
    let right_down = Position(2, 2);

    assert!(left_up < right_down);
    assert!(left_up == left_up);
    assert!(right_down > left_up);
}

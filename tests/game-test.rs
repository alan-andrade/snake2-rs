extern crate grid;

use grid::{Grid, Position, AllocationEvent};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Object {
    Snake,
    Apple,
    Wall
}

struct Game<'a, T: 'a> {
    grid: &'a mut Grid<T>
}

impl<'a, T> Game<'a, T> where T: Collidable, T: Copy {
    fn allocate_at(&mut self, position: Position, object: T) -> AllocationEvent<T> {
        match self.grid.allocate_object_at(object, position) {
            AllocationEvent::Collition(obstacle) => {
                object.handle_collition(obstacle)
            },

            AllocationEvent::Allocated => {
                AllocationEvent::Allocated
            },

            _ => { AllocationEvent::Crash }
        }
    }
}

trait Collidable {
    fn handle_collition(&self, Self) -> AllocationEvent<Self>;
}

impl Collidable for Object {
    fn handle_collition(&self, obstacle: Object) -> AllocationEvent<Object> {
        match (self, obstacle) {
            (&Object::Snake, Object::Snake) |
            (&Object::Snake, Object::Wall) => {return AllocationEvent::Crash; }
            (&Object::Snake, Object::Apple) => { return AllocationEvent::Yum; }
            (_, _) => { return AllocationEvent::CollitionRuleMissing }
        }
    }
}

#[test]
fn game_has_a_grid() {
    let mut grid = Grid::new(4, 4);
    let mut game = Game { grid: &mut grid };
    let (wall, apple, snake) = (Object::Wall, Object::Apple, Object::Snake);

    assert_eq!(game.allocate_at(Position(1, 1), wall), AllocationEvent::Allocated);
    assert_eq!(game.allocate_at(Position(1, 2), apple), AllocationEvent::Allocated);
    assert_eq!(game.allocate_at(Position(1, 2), snake), AllocationEvent::Yum);
    assert_eq!(game.allocate_at(Position(1, 1), snake), AllocationEvent::Crash);
    assert_eq!(game.allocate_at(Position(1, 3), snake), AllocationEvent::Allocated);
    assert_eq!(game.allocate_at(Position(1, 3), snake), AllocationEvent::Crash);
    assert_eq!(game.allocate_at(Position(1, 1), apple), AllocationEvent::CollitionRuleMissing);
}

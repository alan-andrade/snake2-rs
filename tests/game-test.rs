extern crate grid;

use grid::{Grid, Object, Position, AllocationEvent};

struct Game<'a> {
    grid: &'a mut Grid
}

impl<'a> Game<'a> {
    fn allocate_at(&mut self, position: Position, object: Object) -> AllocationEvent {
        match self.grid.allocate_at(position, object) {
            AllocationEvent::Collition(obstacle) => {
                // Object conforms to Collidable to be able to
                // handle collitions.
                object.handle_collition(&obstacle)
            },

            AllocationEvent::Allocated => {
                AllocationEvent::Allocated
            },

            _ => { AllocationEvent::Crash }
        }
    }
}

trait Collidable {
    fn handle_collition(&self, &Object) -> AllocationEvent;
}

impl Collidable for Object {
    fn handle_collition(&self, obstacle: &Object) -> AllocationEvent {
        match (self, obstacle) {
            (&Object::Snake, &Object::Snake) |
            (&Object::Snake, &Object::Wall) => {return AllocationEvent::Crash; }
            (&Object::Snake, &Object::Apple) => { return AllocationEvent::Yum; }
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

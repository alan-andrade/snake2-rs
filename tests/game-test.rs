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

#[derive(Debug, PartialEq, Eq)]
enum GameEvent {
    Crash,
    ObjectAdded
}

impl<'a, T> Game<'a, T> {
    fn add_object_at(&mut self, object: T, position: Position) -> GameEvent {
        match self.grid.allocate_object_at(object, position) {
            AllocationEvent::Collition(obstacle) => { GameEvent::Crash },
            AllocationEvent::Allocated => { GameEvent::ObjectAdded },
            _ => { GameEvent::Crash }
        }
    }
}

#[test]
fn game_has_a_grid() {
    let mut grid = Grid::new(4, 4);
    let mut game = Game { grid: &mut grid };
    let (wall, apple, snake) = (Object::Wall, Object::Apple, Object::Snake);

    assert_eq!(game.add_object_at(wall, Position(1, 1)), GameEvent::ObjectAdded);
    assert_eq!(game.add_object_at(apple, Position(1, 2)), GameEvent::ObjectAdded);
    //assert_eq!(game.add_object_at(Position(1, 2), snake), AllocationEvent::Yum);
    //assert_eq!(game.add_object_at(Position(1, 1), snake), AllocationEvent::Crash);
    assert_eq!(game.add_object_at(snake, Position(1, 3)), GameEvent::ObjectAdded);
    //assert_eq!(game.add_object_att(Position(1, 3), snake), AllocationEvent::Crash);
    //assert_eq!(game.add_object_att(Position(1, 1), apple), AllocationEvent::CollitionRuleMissing);
}

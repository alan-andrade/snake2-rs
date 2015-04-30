extern crate grid;

use grid::{Grid, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Object {
    Snake,
    Apple,
    Wall
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    body: Vec<Position>,
    direction: Direction
}

impl Player {
    fn head(&self) -> Option<&Position> {
        self.body.last()
    }

    fn move_to(&mut self, position: Position) -> Position {
        self.body.push(position);
        self.body.remove(0);
        return position;
    }
}

fn navigate(position: &Position, direction: Direction) -> Position {
    let Position(x, y) = *position;

    match direction {
        Direction::Up    => Position(x  , y-1),
        Direction::Down  => Position(x  , y+1),
        Direction::Right => Position(x+1, y  ),
        Direction::Left  => Position(x-1, y  )
    }
}

fn setup_walls(grid: &mut Grid<Object>) {

    let mut to_be_occupied = vec!();

    // XXX: Lazy generate positions
    {
        let mut top_wall = Position(1, 1);
        let mut bottom_wall = Position(1, grid.height);

        for _ in (1..grid.width + 1) {
            to_be_occupied.push(top_wall);
            top_wall = navigate(&top_wall, Direction::Right);

            to_be_occupied.push(bottom_wall);
            bottom_wall = navigate(&bottom_wall, Direction::Right);
        }
    }

    {
        let mut left_wall = Position(1, 1);
        let mut right_wall= Position(grid.width, 1);

        for _ in (1 .. grid.height + 1) {
            to_be_occupied.push(left_wall);
            left_wall = navigate(&left_wall, Direction::Down);

            to_be_occupied.push(right_wall);
            right_wall = navigate(&right_wall, Direction::Down);
        }
    }

    for position in to_be_occupied {
        grid.allocate_object_at(Object::Wall, position);
    }
}

#[test]
fn test_setup_walls() {
    let mut grid = Grid::new(5, 5);
    setup_walls(&mut grid);
    assert_eq!(grid.object_at(Position(1, 1)), Some(&Object::Wall));
    assert_eq!(grid.object_at(Position(5, 5)), Some(&Object::Wall));
    assert_eq!(grid.object_at(Position(1, 5)), Some(&Object::Wall));
    assert_eq!(grid.object_at(Position(5, 1)), Some(&Object::Wall));
    assert_eq!(grid.object_at(Position(3, 3)), None);
}

struct Controller<'a> {
    grid: &'a mut Grid<Object>
}

impl<'a> Controller<'a> {
    fn add_player(&mut self) -> Player {
        use grid::AllocationEvent::Allocated;

        let mut body = vec!();
        let mut position = self.grid.center();
        for _ in (1..4) {
            position = match self.grid.allocate_object_at(Object::Snake, position) {
                Allocated => {
                    body.push(position);
                    navigate(&position, Direction::Up)
                }
                _ => panic!("Position {} is not available, Couldn't allocate snake", position)
            }
        }

        return Player { body: body, direction: Direction::Down }
    }

    fn move_player(&mut self, player: &mut Player, direction: Direction) -> bool {
        use grid::AllocationEvent::*;

        // player needs to know dafuq positions it was provided.
        let next_position = navigate(player.head().unwrap(), direction);
        self.grid.free(&player.move_to(next_position));

        match self.grid.allocate_object_at(Object::Snake, next_position) {
            Allocated => true,
            _ => false
        }
    }
}

#[test]
fn test_controller() {
    let mut grid = Grid::new(4, 10);
    let mut controller = Controller { grid: &mut grid };

    let mut player = controller.add_player();
    assert!(controller.move_player(&mut player, Direction::Right));
    assert!(controller.move_player(&mut player, Direction::Right));
    // CRASH HERE against a wall
    assert!(!controller.move_player(&mut player, Direction::Right));
}

fn main () { }

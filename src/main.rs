extern crate grid;

use grid::{Grid, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Object {
    Snake,
    Apple,
    Wall
}

impl Object {
    fn foo() {}
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug, PartialEq, Eq)]
struct Player(Position, Object);

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

fn setup_snake(grid: &mut Grid<Object>) {
    use grid::AllocationEvent::Allocated;

    let mut position = grid.center();
    for _ in (1..4) {
        position = match grid.allocate_object_at(Object::Snake, position) {
            Allocated => navigate(&position, Direction::Up),
            _ => panic!("Position {} is not available, Couldn't allocate snake", position)
        }
    }
}

fn setup_grid(grid: &mut Grid<Object>) {
    setup_walls(grid);
    setup_snake(grid);
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

#[test]
fn test_setup_grid() {
    let mut grid = Grid::new(10, 10);

    setup_grid(&mut grid);

    assert_eq!(grid.object_at(Position(1, 1)), Some(&Object::Wall));
    let center = grid.center();
    assert_eq!(grid.object_at(center), Some(&Object::Snake));
}

fn main () { }

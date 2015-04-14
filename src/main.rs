extern crate grid;

use grid::{Grid, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Object {
    Snake,
    Apple,
    Wall
}

enum Direction {
    Up,
    Right,
    Down,
    Left
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

struct Stage;

impl Stage {
    fn setup(grid: &mut Grid<Object>) {
        // setup walls
        let mut collector = vec!();

        {
            let mut top_wall = Position(1, 1);
            let mut bottom_wall = Position(1, grid.height);

            for _ in 1..grid.width {
                collector.push(top_wall);
                top_wall = navigate(&top_wall, Direction::Right);

                collector.push(bottom_wall);
                bottom_wall = navigate(&bottom_wall, Direction::Right);
            }
        }

        {
            let mut left_wall = Position(1, 1);
            let mut right_wall= Position(grid.width, 1);

            for _ in 1..grid.height {
                collector.push(left_wall);
                left_wall = navigate(&left_wall, Direction::Down);

                collector.push(right_wall);
                right_wall = navigate(&right_wall, Direction::Down);
            }
        }

        for pos in collector {
            grid.allocate_object_at(Object::Wall, pos);
        }

        //let center = grid.center();
        //for _ in (1..4) {
            //position = match grid.allocate_object_at(Snake, position) {
                //Allocated => navigate(&position, Direction::Up),
                //_ => panic!("Position {} is not available, Couldn't allocate snake", position)
            //}
        //}
    }
}


#[test]
fn test_stage_setup() {
    let mut grid = Grid::new(4, 4);
    Stage::setup(&mut grid);

    assert_eq!(grid.object_at(Position(1, 1)), Some(&Object::Wall));
}

fn main () { }

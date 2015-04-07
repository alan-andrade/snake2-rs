extern crate grid;

use grid::{Grid, Position, Object};
use grid::AllocationEvent::{MoveSourceEmpty, Allocated};

#[test]
fn grid_can_move() {
    let mut grid = Grid::new(10, 10);
    let top_left = Position(1, 1);
    let bottom_right = Position(10, 10);

    assert_eq!(grid.move_object(top_left, bottom_right), MoveSourceEmpty);
    assert_eq!(grid.allocate_at(top_left, Object::Snake), Allocated);
    assert_eq!(grid.move_object(top_left, bottom_right), Allocated);
}

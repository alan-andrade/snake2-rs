extern crate grid;

use grid::{Grid, Position, Object};
use grid::AllocationEvent::{EmptySpace, Allocated};

#[test]
fn grid_motion() {
    let mut grid = Grid::new(10, 10);
    let top_left = Position(1, 1);
    let bottom_right = Position(10, 10);

    assert_eq!(grid.move_object(top_left, bottom_right), EmptySpace);
    assert_eq!(grid.allocate_at(top_left, Object::Snake), Allocated);
    assert_eq!(grid.move_object(top_left, bottom_right), Allocated);

    if let Some(object) = grid.object_at(bottom_right) {
        assert_eq!(object, &Object::Snake);
    } else {
        panic!();
    }

    assert_eq!(grid.object_at(top_left), None);
}

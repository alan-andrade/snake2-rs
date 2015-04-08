extern crate grid;

use grid::{Grid, Position};
use grid::AllocationEvent::{Collition, EmptySpace, Allocated};

#[test]
fn grid_motion() {
    let mut grid = Grid::new(10, 10);
    let top_left = Position(1, 1);
    let bottom_right = Position(10, 10);
    let object = Some("obj");

    assert_eq!(grid.move_object(top_left, bottom_right), EmptySpace);
    assert_eq!(grid.allocate_object_at(object, top_left), Allocated);
    assert_eq!(grid.move_object(top_left, bottom_right), Allocated);
    assert_eq!(grid.move_object(top_left, bottom_right), EmptySpace);
    if let Some(object) = grid.object_at(bottom_right) {
        assert_eq!(object, object);
    } else {
        panic!();
    }
    assert_eq!(grid.object_at(top_left), None);

    assert_eq!(grid.allocate_object_at(object, top_left), Allocated);
    assert_eq!(grid.move_object(bottom_right, top_left), Collition(object));
}

struct Grid {
    width: u8,
    height: u8
}

impl Grid {
    fn allocate (&mut self, object: Object) {
        // todo
    }

    fn occupied_count (&self) -> u8 {
        return 2
    }
}

struct Object;

#[test]
fn grid_allocate () {
    let mut grid = Grid { width: 5, height: 5};

    let foo = Object;
    let bar = Object;

    grid.allocate(foo);
    grid.allocate(bar);

    assert_eq!(grid.occupied_count(), 2)//, "2 objects occupy the grid");
}

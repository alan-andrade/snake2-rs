
static SNAKE: char = '#';

trait Drawable { fn draw(&self); }

struct Snake {
    skin: char
}

impl Snake {
    fn new () -> Snake {
        Snake { skin: SNAKE }
    }
}

impl Drawable for Snake {
    fn draw (&self) { println!("{}", self.skin); }
}

fn main() {
    let s = Snake::new();
    s.draw();
}

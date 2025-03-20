fn main() {
    const WIDTH: i32 = 25;
    const HEIGHT: i32 = 10;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let is_horizontal = y == 0 || y == HEIGHT - 1;
            let is_vertical = x == 0 || x == WIDTH - 1;
            let is_diagonal = y == (HEIGHT * x) / WIDTH || y == (HEIGHT * (WIDTH - x - 1)) / WIDTH;
            let show = is_horizontal || is_vertical || is_diagonal;
            let symbol = if show { '*' } else { ' ' };
            print!("{}", symbol);
        }
        println!();
    }
}






//task: https://classroom.google.com/u/3/c/Njg3NDI3OTc3OTY1/a/NzUwOTA2ODc2ODQx/details
fn main() {
    const WIDTH: i32 = 11;
    const HEIGHT: i32 = 11;
    let mid = HEIGHT / 2;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let num_stars = if y <= mid {
                2 * y + 1
            } else {
                2 * (HEIGHT - y - 1) + 1
            };
            let num_spaces = (WIDTH - num_stars) / 2;
            if x >= num_spaces && x < num_spaces + num_stars {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
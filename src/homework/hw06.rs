fn main() {
    let layers = 3;

    (0..layers)
        .flat_map(|layer|{
            let height = layer+1;
            (0..height)
                .map(move|i|{
                    let max_width = 2 * layers + 1;
                    let stars_count = 2 * i + 1;
                    let spaces_count = (max_width - stars_count) / 2;

                    let spaces = (0..spaces_count)
                        .map(|_| ' ')
                        .collect::<String>();
                    let stars = (0..stars_count)
                        .map(|_| '*')
                        .collect::<String>();

                    format!("{}{}", spaces, stars)
                })
        })
        .for_each(|line| println!("{}", line));
}
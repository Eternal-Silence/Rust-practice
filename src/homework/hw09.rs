fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s.to_string();
    }

    // Нормалізуємо зсув у межах [0, len)
    let shift = ((shift % len as isize + len as isize) % len as isize) as usize;

    // Здійснюємо правий зсув (цикл вправо)
    let split_point = len - shift;
    format!("{}{}", &s[split_point..], &s[..split_point])
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s, n),
                exp.to_string()
            )
        );
}

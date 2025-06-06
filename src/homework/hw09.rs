fn rotate(s: &str, shift: isize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    if len == 0 {
        return String::new();
    }

    let shift = ((shift % len as isize + len as isize) % len as isize) as usize;
    let split_point = len - shift;

    chars[split_point..]
        .iter()
        .chain(&chars[..split_point])
        .collect()
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
                rotate(s, *n),
                exp.to_string()
            )
        );
}

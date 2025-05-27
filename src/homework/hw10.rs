fn is_palindrome(x: u32) -> bool {
    let n = x.to_string();
    n.chars().eq(n.chars().rev())
}

#[test]
fn test() {
    let data =
        [
            (123, false),
            (121, true),
            (1221, true),
        ];


    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}

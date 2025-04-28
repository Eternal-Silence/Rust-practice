fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a > b{
        while b != 0{
            let temp = b;
            b = a % b;
            a = temp
        }
        return a
    }
    else {
        while a != 0{
            let temp = a;
            a = b % a;
            b = temp
        }
        return b
    }
}

#[test]
fn test() {
    let data =
        [
            ((24,  60), 12),
            ((15,   9),  3),
            ((15,   6),  3),
            ((140, 40), 20),
            ((24,  16),  8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37,  11),  1),
            ((120, 90), 30),
        ];


    for ((a, b), exp) in data.iter() {
        assert_eq!(*exp, gcd(*a, *b));
    }
}

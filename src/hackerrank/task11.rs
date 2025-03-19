// task: https://www.hackerrank.com/challenges/apple-and-orange/problem
fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {

    let mut apple_count = 0;

    let mut orange_count = 0;

    for delta in apples {
        let pos = a + delta;
        if pos >= s && pos <= t{
            apple_count +=1;
        }
    }
    for delta in oranges {
        let pos = b + delta;
        if pos >= s && pos <= t{
            orange_count +=1;
        }
    }
    println!("{}\n{}", apple_count, orange_count);

}

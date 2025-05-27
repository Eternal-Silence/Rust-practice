/* Even numbers are always
а - 2
н - 4
о - 6
х - 8
There should be 2 correct solutions as far as i am aware. But i'll leave that to a computer.
Rules:
Digits are 1-8.
Number must be 4 digits and all the digits in both numbers are unique.
First number is always multiplied by its fourth digit.
 */

fn digits_are_valid(digits: &[u32]) -> bool {
    let mut seen = [false; 9];
    for &d in digits {
        if d < 1 || d > 8 || seen[d as usize] {
            return false;
        }
        seen[d as usize] = true;
    }
    true
}

fn main() {
    let mut correct_solutions = 0;

    for d1 in 1..=8 {
        for d2 in 1..=8 {
            for d3 in 1..=8 {
                for d4 in 1..=8 {
                    let original_digits = [d1, d2, d3, d4];
                    if !digits_are_valid(&original_digits) {
                        continue;
                    }

                    let number = d1 * 1000 + d2 * 100 + d3 * 10 + d4;
                    let multiplier = d4;
                    let result = number * multiplier;

                    if result < 1000 || result > 9999 {
                        continue;
                    }

                    let r1 = result / 1000;
                    let r2 = (result / 100) % 10;
                    let r3 = (result / 10) % 10;
                    let r4 = result % 10;

                    let result_digits = [r1, r2, r3, r4];

                    if !digits_are_valid(&result_digits) {
                        continue;
                    }

                    if result_digits.iter().any(|&d| original_digits.contains(&d)) {
                        continue;
                    }

                    correct_solutions += 1;

                    println!("{:>4}", number);
                    println!("x{:>3}", multiplier);
                    println!("----");
                    println!("{:>4}\n", result);
                }
            }
        }
    }

    println!("\nTotal correct solutions: {}", correct_solutions);
}

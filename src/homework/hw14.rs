fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let size = 1 << n;
    (0..size)
        .map(|i| {
            let gray = i ^ (i >> 1);
            format!("{:0width$b}", gray, width = n as usize)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!["".to_string()]),
            (1, vec!["0", "1"].iter().map(|s| s.to_string()).collect()),
            (2, vec!["00", "01", "11", "10"].iter().map(|s| s.to_string()).collect()),
            (3, vec![
                "000", "001", "011", "010",
                "110", "111", "101", "100"
            ].iter().map(|s| s.to_string()).collect()),
            (4, vec![
                "0000", "0001", "0011", "0010",
                "0110", "0111", "0101", "0100",
                "1100", "1101", "1111", "1110",
                "1010", "1011", "1001", "1000"
            ].iter().map(|s| s.to_string()).collect()),
        ];

        test_data.iter().for_each(|(n, out)| {
            assert_eq!(gray(*n), *out)
        });
    }
}

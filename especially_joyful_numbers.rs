fn number_joy(n: u32) -> bool {
    let s = n.to_string();

    let mut iter = s.chars();
    let mut sum = 0;
    while let Some(c) = iter.next() {
        sum += c.to_digit(10).unwrap();
    }

    let sum_rev = sum
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    sum * sum_rev == n
}

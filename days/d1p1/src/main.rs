fn main() {
    let input = include_str!("../input.txt");
    let val: u32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<char> = line.chars().filter(|ch| ch.is_numeric()).collect();
            let first = numbers.first().unwrap_or(&'0');
            let last = numbers.last().unwrap_or(&'0');
            let sum_str: String = vec![first, last].into_iter().collect();
            sum_str.parse::<u32>().unwrap()
        })
        .sum();
    dbg!(val);
}

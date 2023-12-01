fn main() {
    let input = include_str!("../input.txt");
    let val: u32 = input
        .lines()
        .map(|line| {
            let chars_forward: String = line.chars().collect();
            let forward_lookup = [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ];
            let first_word_as_num = forward_lookup
                .into_iter()
                .filter_map(|(num_str, digit)| {
                    if let Some(idx) = chars_forward.find(num_str) {
                        return Some((idx, num_str, digit));
                    }
                    None
                })
                .min_by(|a, b| a.0.cmp(&b.0));
            let replaced_forward_line = if let Some(val) = first_word_as_num {
                chars_forward.replacen(val.1, &val.2.to_string(), 1)
            } else {
                chars_forward
            };

            let forward_numbers: Vec<char> = replaced_forward_line
                .chars()
                .filter(|ch| ch.is_numeric())
                .collect();
            let first = forward_numbers.first().unwrap_or(&'0');

            let chars_backward: String = line.chars().rev().collect();
            let backward_lookup = [
                ("eno", 1),
                ("owt", 2),
                ("eerht", 3),
                ("ruof", 4),
                ("evif", 5),
                ("xis", 6),
                ("neves", 7),
                ("thgie", 8),
                ("enin", 9),
            ];
            let first_word_as_num_backward = backward_lookup
                .into_iter()
                .filter_map(|(num_str, digit)| {
                    if let Some(idx) = chars_backward.find(num_str) {
                        return Some((idx, num_str, digit));
                    }
                    None
                })
                .min_by(|a, b| a.0.cmp(&b.0));

            let replaced_backward_line = if let Some(val) = first_word_as_num_backward {
                chars_backward.replacen(val.1, &val.2.to_string(), 1)
            } else {
                chars_backward
            };

            let backward_numbers: Vec<char> = replaced_backward_line
                .chars()
                .filter(|ch| ch.is_numeric())
                .collect();
            let last = backward_numbers.first().unwrap_or(&'0');

            let sum_str: String = vec![first, last].into_iter().collect();
            sum_str.parse::<u32>().unwrap()
        })
        .sum();
    dbg!(val);
}

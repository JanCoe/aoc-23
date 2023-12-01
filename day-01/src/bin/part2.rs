fn main() {
    let content = include_str!("../../../input/day-01.txt");
    let total = calc_total(content);
    println!("Total: {}", total);
}

fn calc_total(content: &str) -> u32 {
    let mut total = 0;
    for line in content.lines() {
        let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let words_backwards = vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
        let first = find_first(line, &words);
        let last = find_last(line, &words_backwards);
        total += first*10 + last;
    }
    total
}

fn find_first(line: &str, words: &Vec<&str>) -> u32 {
    let mut index: Option<usize> = None;
    let mut digit = 0;

    //Find index of first digit
    for (counter, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            digit = character.to_digit(10).unwrap();
            index = Some(counter);
            break;
        }
    }

    //Find index of spelt out digit and capture it if it is lower than current first digit index
    for (counter, word) in words.into_iter().enumerate() {
        if let Some(word_index) = line.find(word) {
            match index {
                None => index = Some(word_index),
                Some(first_so_far) => {
                    if word_index < first_so_far {
                        index = Some(word_index);
                        digit = (counter + 1) as u32;
                    }
                }
            }
        }
    }
    digit
}

fn find_last(line: &str, words_backwards: &Vec<&str>) -> u32 {
    let line = line.chars().rev().collect::<String>();
    find_first(&line, &words_backwards)
}

#[cfg(test)]
mod tests {
    use super::{find_first, find_last, calc_total};

    #[test]
    fn test_find_first() {
        let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        assert_eq!(find_first("two1nine", &words), 2);
        assert_eq!(find_first("eighttwothree", &words), 8);
        assert_eq!(find_first("abcone2threexyz", &words), 1);
        assert_eq!(find_first("xtwone3four", &words), 2);
        assert_eq!(find_first("4nineeightseven2", &words), 4);
        assert_eq!(find_first("zoneight234", &words), 1);
        assert_eq!(find_first("7pqrstsixteen", &words), 7);
    }

    #[test]
    fn test_find_last() {
        let words = vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
        assert_eq!(find_last("two1nine", &words), 9);
        assert_eq!(find_last("eighttwothree", &words), 3);
        assert_eq!(find_last("abcone2threexyz", &words), 3);
        assert_eq!(find_last("xtwone3four", &words), 4);
        assert_eq!(find_last("4nineeightseven2", &words), 2);
        assert_eq!(find_last("zoneight234", &words), 4);
        assert_eq!(find_last("7pqrstsixteen", &words), 6);
        assert_eq!(find_last("sxbjdbtlnjrmlzgxneightthreepmqxdxhfk8jfrheightwovp", &words), 2);
        assert_eq!(find_last("sevenonezknqnkfqbzffjvfivetwo94two", &words), 2);
    }

    #[test]
    fn test_calc_total() {
        let content = include_str!("../../../input/day-01_part2_test.txt");
        let total = calc_total(content);
        assert_eq!(total, 281);
    }
}

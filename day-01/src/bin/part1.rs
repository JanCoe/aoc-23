fn main() {
    let content = include_str!("../../../input/day-01.txt");
    let total = calc_total(content);
    println!("Total: {}", total);
}

fn calc_total(content: &str) -> u32 {
    let mut total = 0;
    for line in content.lines() {
        let first = find_digit(line, false);
        let last = find_digit(line, true);
        total += first*10 + last;
    }
    total
}
fn find_digit(line: &str, reverse: bool) -> u32 {
    let mut digit = 0;

    if reverse {
        for c in line.chars().rev() {
            if c.is_digit(10) {
                digit = c.to_digit(10).unwrap();
                break;
            }
        }
    } else {
        for c in line.chars() {
            if c.is_digit(10) {
                digit = c.to_digit(10).unwrap();
                break;
            }
        }
    }
    digit
}

#[cfg(test)]
mod tests {
    use super::{find_digit, calc_total};

    #[test]
    fn test_find_digit() {
        assert_eq!(find_digit("treb7uchet", false), 7);
        assert_eq!(find_digit("treb7uchet", true), 7);
    }

    #[test]
    fn test_calc_total() {
        let content = include_str!("../../../input/day-01_test.txt");
        assert_eq!(calc_total(content), 142);
    }
}

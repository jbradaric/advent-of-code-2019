use std::collections::HashMap;
use std::env;

fn acceptable(num: u64) -> bool {
    let s = num.to_string();
    if s.len() != 6 {
        return false;
    }
    let mut adjacent_same = false;
    let mut adjacent = HashMap::new();
    for (d1, d2) in s.chars().zip(s.chars().skip(1)) {
        let n1 = d1.to_digit(10).unwrap();
        let n2 = d2.to_digit(10).unwrap();
        if n1 == n2 {
            let counter = adjacent.entry(n1).or_insert(1);
            *counter += 1;
            adjacent_same = true;
        } else if n2 < n1 {
            return false;
        }
    }
    adjacent_same
        && ((adjacent.len() > 1
                && !adjacent.values().all(|&x| x == 3))
            || adjacent.values().filter(|n| **n == 2).count() == 1)
}

fn main() {
    let start: u64 = env::args().nth(1).map(|s| s.parse().unwrap()).unwrap();
    let end: u64 = env::args().nth(2).map(|s| s.parse().unwrap()).unwrap();
    let c = (start..end).filter(|n| acceptable(*n)).count();
    dbg!(c);
}

#[cfg(test)]
mod test {
    use super::acceptable;

    #[test]
    fn test_1() {
        assert!(!acceptable(111111));
        assert!(!acceptable(223450));
        assert!(!acceptable(123789));
    }

    #[test]
    fn test_2() {
        assert!(acceptable(112233));
        assert!(!acceptable(123444));
        assert!(acceptable(111122));
    }
}

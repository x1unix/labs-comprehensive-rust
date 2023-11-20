// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let nums = cc_number
        .split_whitespace()
        .flat_map(|str| str.trim().chars())
        .map(|c| c.to_digit(10))
        .rev()
        // .filter_map(|num| num)
        .fold(
            Some(Vec::with_capacity(cc_number.len())),
            |acc, val| match val {
                Some(v) => {
                    if let Some(mut arr) = acc {
                        arr.push(v);
                        Some(arr)
                    } else {
                        None
                    }
                }
                None => None,
            },
        );

    match nums {
        Some(nums) if nums.len() > 0 => {
            nums.into_iter()
                .enumerate()
                .map(|(i, num)| {
                    if (i + 1) % 2 > 0 {
                        return num;
                    }

                    let mut sum = num * 2;
                    if sum > 9 {
                        sum = (sum % 10) + (sum / 10);
                    }

                    sum
                })
                .reduce(|sum, val| sum + val)
                .unwrap_or(0)
                % 10
                == 0
        }
        _ => false,
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
pub fn main() {}

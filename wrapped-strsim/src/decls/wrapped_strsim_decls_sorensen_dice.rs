use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Calculates a Sørensen-Dice similarity distance using bigrams.
/// See <https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.
///
/// ```
/// use strsim::sorensen_dice;
///
/// assert_eq!(1.0, sorensen_dice("", ""));
/// assert_eq!(0.0, sorensen_dice("", "a"));
/// assert_eq!(0.0, sorensen_dice("french", "quebec"));
/// assert_eq!(1.0, sorensen_dice("ferris", "ferris"));
/// assert_eq!(0.8888888888888888, sorensen_dice("feris", "ferris"));
/// ```
pub fn sorensen_dice(a: &str, b: &str) -> f64 {
    let a: String = a.chars().filter(|&x| !char::is_whitespace(x)).collect();
    let b: String = b.chars().filter(|&x| !char::is_whitespace(x)).collect();
    if a == b {
        return 1.0;
    }
    if a.len() < 2 || b.len() < 2 {
        return 0.0;
    }
    let mut a_bigrams: HashMap<(char, char), usize> = HashMap::new();
    for bigram in bigrams(&a) {
        *a_bigrams.entry(bigram).or_insert(0) += 1;
    }
    let mut intersection_size = 0_usize;
    for bigram in bigrams(&b) {
        a_bigrams
            .entry(bigram)
            .and_modify(|bi| {
                if *bi > 0 {
                    *bi -= 1;
                    intersection_size += 1;
                }
            });
    }
    (2 * intersection_size) as f64 / (a.len() + b.len() - 2) as f64
}

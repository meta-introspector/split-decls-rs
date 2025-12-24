use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns an Iterator of char tuples.
fn bigrams(s: &str) -> impl Iterator<Item = (char, char)> + '_ {
    s.chars().zip(s.chars().skip(1))
}

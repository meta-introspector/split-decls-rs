use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Like Jaro but gives a boost to strings that have a common prefix.
///
/// ```
/// use strsim::jaro_winkler;
///
/// assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() <
///         0.001);
/// ```
pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    generic_jaro_winkler(&StringWrapper(a), &StringWrapper(b))
}

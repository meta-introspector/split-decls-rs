use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn sort_by_words (name : & str) -> Vec < & str > { let mut split_words : Vec < & str > = name . split ('_') . collect () ; split_words . sort_unstable () ; split_words }
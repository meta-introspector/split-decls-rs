use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn nth < I > (n : u8) -> I where I : Zero + One , { let mut i = I :: zero () ; for _ in 0 .. n { i = i + I :: one () ; } i }
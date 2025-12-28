use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone)] enum Diff < 'a , 'b > { Equal (Range < 'a > , Range < 'b >) , Delete (Range < 'a >) , Insert (Range < 'b >) , }
}
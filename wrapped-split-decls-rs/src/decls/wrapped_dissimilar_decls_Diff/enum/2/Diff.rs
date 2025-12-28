use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone)] enum Diff < 'a , 'b > { Equal (Range < 'a > , Range < 'b >) , Delete (Range < 'a >) , Insert (Range < 'b >) , }
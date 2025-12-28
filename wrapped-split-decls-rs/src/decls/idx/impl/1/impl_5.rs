use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Idx for usize { # [inline] fn new (idx : usize) -> Self { idx } # [inline] fn index (self) -> usize { self } }
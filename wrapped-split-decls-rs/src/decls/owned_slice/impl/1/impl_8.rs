use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Borrow < [u8] > for OwnedSlice { # [inline] fn borrow (& self) -> & [u8] { self } }
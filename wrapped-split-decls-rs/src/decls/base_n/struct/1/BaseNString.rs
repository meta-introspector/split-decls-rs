use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BaseNString { start : usize , buf : [ascii :: Char ; 128] , }
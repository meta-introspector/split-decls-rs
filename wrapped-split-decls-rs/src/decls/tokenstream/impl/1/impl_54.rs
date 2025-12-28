use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl DelimSpacing { pub fn new (open : Spacing , close : Spacing) -> DelimSpacing { DelimSpacing { open , close } } }
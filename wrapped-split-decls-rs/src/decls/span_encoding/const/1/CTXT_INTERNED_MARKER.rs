use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const CTXT_INTERNED_MARKER : u16 = 0b1111_1111_1111_1111 ;
}
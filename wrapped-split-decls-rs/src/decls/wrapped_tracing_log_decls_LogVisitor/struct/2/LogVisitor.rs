use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct LogVisitor < 'a > { target : Option < & 'a str > , module_path : Option < & 'a str > , file : Option < & 'a str > , line : Option < u64 > , fields : & 'static Fields , }
}
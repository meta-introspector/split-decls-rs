use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct RecursiveGuard (& 'static LocalKey < AtomicBool >) ;
}
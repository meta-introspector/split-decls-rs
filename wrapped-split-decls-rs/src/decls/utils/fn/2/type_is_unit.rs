use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Checks whether the type `ty` is `()`."] pub (crate) fn type_is_unit (ty : & Type) -> bool { if let Type :: Tuple (TypeTuple { elems , .. }) = ty { elems . is_empty () } else { false } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn find_by_name < A : AttributeExt > (attrs : & [A] , name : Symbol) -> Option < & A > { filter_by_name (attrs , name) . next () }
}
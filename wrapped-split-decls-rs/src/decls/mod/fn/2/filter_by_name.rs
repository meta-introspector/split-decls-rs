use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn filter_by_name < A : AttributeExt > (attrs : & [A] , name : Symbol) -> impl Iterator < Item = & A > { attrs . iter () . filter (move | attr | attr . has_name (name)) }
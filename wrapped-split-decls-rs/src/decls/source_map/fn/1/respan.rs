use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn respan < T > (sp : Span , t : T) -> Spanned < T > { Spanned { node : t , span : sp } }
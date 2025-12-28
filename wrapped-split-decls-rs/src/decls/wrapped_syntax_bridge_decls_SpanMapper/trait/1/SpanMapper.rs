use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait SpanMapper < S > { fn span_for (& self , range : TextRange) -> S ; }
}
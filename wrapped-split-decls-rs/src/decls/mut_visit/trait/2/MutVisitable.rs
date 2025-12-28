use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub (crate) trait MutVisitable < V : MutVisitor > { type Extra : Copy ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) ; }
}
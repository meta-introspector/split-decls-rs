use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait TupleExt { type Head ; type Tail ; fn head (self) -> Self :: Head ; fn tail (self) -> Self :: Tail ; }
}
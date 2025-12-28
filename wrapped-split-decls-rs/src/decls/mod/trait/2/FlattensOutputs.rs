use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait FlattensOutputs : Sized { type FlatMapOutputTy ; fn flatten_outputs (_outputs : impl Iterator < Item = Self :: FlatMapOutputTy >) -> Self :: FlatMapOutputTy ; }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait FlattensOutputs : Sized { type FlatMapOutputTy ; fn flatten_outputs (_outputs : impl Iterator < Item = Self :: FlatMapOutputTy >) -> Self :: FlatMapOutputTy ; }
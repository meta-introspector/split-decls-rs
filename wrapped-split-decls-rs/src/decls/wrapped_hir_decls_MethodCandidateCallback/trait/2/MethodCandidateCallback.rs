use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait MethodCandidateCallback { fn on_inherent_method (& mut self , f : Function) -> ControlFlow < () > ; fn on_trait_method (& mut self , f : Function) -> ControlFlow < () > ; }
}
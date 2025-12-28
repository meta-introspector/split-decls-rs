use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc = " Small helper trait for abstracting over `Option` fields that contain a value and a `Span`"] # [doc = " for error reporting if they are set more than once."] pub (crate) trait SetOnce < T > { fn set_once (& mut self , value : T , span : Span) ; fn value (self) -> Option < T > ; fn value_ref (& self) -> Option < & T > ; }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
# [doc (hidden)] pub trait ReturnDefault < O > { fn maybe_return_default () -> Option < O > ; fn return_default () -> Result < O , & 'static str > ; }
}
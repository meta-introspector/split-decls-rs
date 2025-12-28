use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > WorkerLocal < Vec < T > > { # [doc = " Joins the elements of all the worker locals into one Vec"] pub fn join (self) -> Vec < T > { self . into_inner () . into_iter () . flat_map (| v | v) . collect () } }
}
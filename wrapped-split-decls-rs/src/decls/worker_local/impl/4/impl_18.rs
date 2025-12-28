use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > WorkerLocal < T > { # [doc = " Creates a new worker local where the `initial` closure computes the"] # [doc = " value this worker local should take for each thread in the registry."] # [inline] pub fn new < F : FnMut (usize) -> T > (mut initial : F) -> WorkerLocal < T > { let registry = Registry :: current () ; WorkerLocal { locals : (0 .. registry . 0 . thread_limit . get ()) . map (| i | CacheAligned (initial (i))) . collect () , registry , } } # [doc = " Returns the worker-local values for each thread"] # [inline] pub fn into_inner (self) -> impl Iterator < Item = T > { self . locals . into_vec () . into_iter () . map (| local | local . 0) } }
}
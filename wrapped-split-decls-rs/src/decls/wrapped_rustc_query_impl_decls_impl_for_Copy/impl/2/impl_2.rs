use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx , C : QueryCache , const ANON : bool , const DEPTH_LIMIT : bool , const FEEDABLE : bool > Copy for DynamicConfig < 'tcx , C , ANON , DEPTH_LIMIT , FEEDABLE > { }
}
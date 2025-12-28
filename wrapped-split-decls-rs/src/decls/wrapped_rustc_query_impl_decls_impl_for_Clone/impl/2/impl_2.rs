use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx , C : QueryCache , const ANON : bool , const DEPTH_LIMIT : bool , const FEEDABLE : bool > Clone for DynamicConfig < 'tcx , C , ANON , DEPTH_LIMIT , FEEDABLE > { fn clone (& self) -> Self { * self } }
}
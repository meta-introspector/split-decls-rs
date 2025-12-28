use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct RealHirInfoItem < 'tcx > (pub Item < 'tcx >) ;
}
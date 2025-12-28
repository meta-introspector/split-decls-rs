use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct EnclosingBreakables < 'tcx > { stack : Vec < BreakableCtxt < 'tcx > > , by_id : HirIdMap < usize > , }
}
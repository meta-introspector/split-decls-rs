use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct RustcTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
}
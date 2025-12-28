use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct MockTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
}
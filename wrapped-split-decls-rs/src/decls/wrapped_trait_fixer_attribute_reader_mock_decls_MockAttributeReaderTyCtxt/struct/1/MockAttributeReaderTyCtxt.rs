use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct MockAttributeReaderTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
}
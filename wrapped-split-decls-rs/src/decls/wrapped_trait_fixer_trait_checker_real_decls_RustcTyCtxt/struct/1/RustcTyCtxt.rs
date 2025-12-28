use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RustcTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
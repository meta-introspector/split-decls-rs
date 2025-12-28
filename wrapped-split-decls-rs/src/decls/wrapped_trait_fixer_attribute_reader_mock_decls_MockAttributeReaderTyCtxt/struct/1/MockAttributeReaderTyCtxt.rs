use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MockAttributeReaderTyCtxt < 'tcx > (pub TyCtxt < 'tcx >) ;
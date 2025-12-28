use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BreakableCtxt < 'tcx > { may_break : bool , coerce : Option < DynamicCoerceMany < 'tcx > > , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct BreakableCtxt < 'tcx > { may_break : bool , coerce : Option < DynamicCoerceMany < 'tcx > > , }
}
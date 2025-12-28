use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn is_mir_available (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { tcx . mir_keys (()) . contains (& def_id) }
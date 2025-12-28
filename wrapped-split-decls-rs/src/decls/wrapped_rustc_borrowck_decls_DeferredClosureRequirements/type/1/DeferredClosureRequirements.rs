use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
type DeferredClosureRequirements < 'tcx > = Vec < (LocalDefId , ty :: GenericArgsRef < 'tcx > , Locations) > ;
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn typeck < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) -> & 'tcx ty :: TypeckResults < 'tcx > { typeck_with_inspect (tcx , def_id , None) }
}
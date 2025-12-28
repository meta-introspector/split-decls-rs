use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct LazyDefPathStr < 'tcx > { def_id : DefId , tcx : TyCtxt < 'tcx > , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct BorrowckInferCtxt < 'tcx > { pub infcx : InferCtxt < 'tcx > , pub root_def_id : LocalDefId , pub param_env : ParamEnv < 'tcx > , pub reg_var_to_origin : RefCell < FxIndexMap < ty :: RegionVid , RegionCtxt > > , }
}
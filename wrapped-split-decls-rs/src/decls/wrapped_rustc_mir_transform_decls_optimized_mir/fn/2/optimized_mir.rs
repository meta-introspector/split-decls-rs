use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Optimize the MIR and prepare it for codegen."] fn optimized_mir (tcx : TyCtxt < '_ > , did : LocalDefId) -> & Body < '_ > { tcx . arena . alloc (inner_optimized_mir (tcx , did)) }
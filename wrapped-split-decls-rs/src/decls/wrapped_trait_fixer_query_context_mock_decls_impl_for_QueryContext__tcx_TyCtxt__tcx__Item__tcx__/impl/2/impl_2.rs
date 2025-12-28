use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'tcx > QueryContext < 'tcx , TyCtxt < 'tcx > , Item < 'tcx > > for MockTyCtxt < 'tcx > { fn walk_hir_tops (& self , _f : impl FnMut (& 'tcx Item < 'tcx >)) { println ! ("Mock MockTyCtxt::walk_hir_tops called") ; } }
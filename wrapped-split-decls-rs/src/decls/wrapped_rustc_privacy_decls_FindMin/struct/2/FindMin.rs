use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Visitor used to determine impl visibility and reachability."] struct FindMin < 'a , 'tcx , VL : VisibilityLike , const SHALLOW : bool > { tcx : TyCtxt < 'tcx > , effective_visibilities : & 'a EffectiveVisibilities , min : VL , }
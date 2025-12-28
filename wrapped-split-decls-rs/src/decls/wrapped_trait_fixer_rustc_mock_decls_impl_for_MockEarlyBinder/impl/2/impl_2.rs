use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'tcx > MockEarlyBinder < 'tcx > { pub fn instantiate (self , _tcx : TyCtxt < 'tcx > , _substs : MockSubsts) -> MockTy < 'tcx > { MockTy (PhantomData) } }
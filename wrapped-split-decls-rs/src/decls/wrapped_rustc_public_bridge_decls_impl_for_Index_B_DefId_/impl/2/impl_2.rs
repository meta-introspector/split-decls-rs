use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'tcx , B : Bridge > Index < B :: DefId > for Tables < 'tcx , B > { type Output = DefId ; # [inline (always)] fn index (& self , index : B :: DefId) -> & Self :: Output { & self . def_ids [index] } }
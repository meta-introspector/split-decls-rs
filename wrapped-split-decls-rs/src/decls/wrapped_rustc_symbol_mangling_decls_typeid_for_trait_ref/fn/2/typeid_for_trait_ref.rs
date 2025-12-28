use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn typeid_for_trait_ref < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: ExistentialTraitRef < 'tcx > ,) -> String { v0 :: mangle_typeid_for_trait_ref (tcx , trait_ref) }
}
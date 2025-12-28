use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'tcx > TyCtxtConsts < 'tcx > { const DEREF_PROJECTION : & 'tcx [PlaceElem < 'tcx > ; 1] = & [ProjectionElem :: Deref] ; }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'tcx > AttributeReader < 'tcx > for RustcTyCtxt < 'tcx > { type DefId = DefId ; type Symbol = Symbol ; fn has_derive_attr (& 'tcx self , def_id : Self :: DefId , trait_name : & str) -> bool { self . 0 . get_attrs (def_id , Self :: sym_derive ()) . flat_map (| attr | attr . meta_item_list () . into_iter () . flatten ()) . any (| item | item . has_name (Self :: sym_intern (trait_name))) } fn sym_derive () -> Self :: Symbol { sym :: derive } fn sym_intern (s : & str) -> Self :: Symbol { Symbol :: intern (s) } }
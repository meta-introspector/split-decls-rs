use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait QueryContext < 'tcx , T , I > where T : Sized + 'tcx , I : Sized + 'tcx , { fn walk_hir_tops (& self , f : impl FnMut (& 'tcx I)) ; }
}
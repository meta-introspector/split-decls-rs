use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait TraitChecker < 'tcx , T , D , Y > where T : Sized + 'tcx , D : Sized + 'tcx , Y : Sized + 'tcx , { fn get_trait_def_id (& self , trait_name : & str) -> Option < D > ; fn type_implements_trait (& self , tcx : T , adt_ty : Y , item_def_id : D , trait_def_id : D) -> bool ; }
}
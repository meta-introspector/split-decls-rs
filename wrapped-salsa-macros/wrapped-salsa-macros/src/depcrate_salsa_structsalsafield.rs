// Generated macro for SalsaField (struct)
macro_rules! Depcrate_salsa_structSalsaField {
() => {
// Module: crate::salsa_struct
// Provides: {"SalsaField"}
// Dependencies: {}
pub (crate) struct SalsaField < 's > { pub (crate) field : & 's syn :: Field , pub (crate) has_tracked_attr : bool , pub (crate) has_default_attr : bool , pub (crate) returns : syn :: Ident , pub (crate) has_no_eq_attr : bool , pub (crate) maybe_update_attr : Option < (syn :: Path , syn :: Expr) > , get_name : syn :: Ident , set_name : syn :: Ident , unknown_attrs : Vec < & 's syn :: Attribute > , }
};
}

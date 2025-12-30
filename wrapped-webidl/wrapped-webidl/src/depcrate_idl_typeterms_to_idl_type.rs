// Generated macro for terms_to_idl_type (macro)
macro_rules! Depcrate_idl_typeterms_to_idl_type {
() => {
// Module: crate::idl_type
// Provides: {"terms_to_idl_type"}
// Dependencies: {}
macro_rules ! terms_to_idl_type { ($ ($ t : tt => $ r : tt) *) => ($ (impl <'a > ToIdlType <'a > for term ::$ t { fn to_idl_type (& self , _record : & FirstPassRecord <'a >) -> IdlType <'a > { IdlType ::$ r } }) *) ; }
};
}

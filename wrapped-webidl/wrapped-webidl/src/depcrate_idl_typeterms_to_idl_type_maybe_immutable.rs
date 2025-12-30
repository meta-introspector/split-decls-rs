// Generated macro for terms_to_idl_type_maybe_immutable (macro)
macro_rules! Depcrate_idl_typeterms_to_idl_type_maybe_immutable {
() => {
// Module: crate::idl_type
// Provides: {"terms_to_idl_type_maybe_immutable"}
// Dependencies: {}
macro_rules ! terms_to_idl_type_maybe_immutable { ($ ($ t : tt => $ r : tt) *) => ($ (impl <'a > ToIdlType <'a > for term ::$ t { fn to_idl_type (& self , _record : & FirstPassRecord <'a >) -> IdlType <'a > { IdlType ::$ r { allow_shared : false , immutable : false } } }) *) ; }
};
}

// Generated macro for crate_def_with_ty (macro)
macro_rules! Depcrate_crate_defcrate_def_with_ty {
() => {
// Module: crate::crate_def
// Provides: {"crate_def_with_ty"}
// Dependencies: {}
macro_rules ! crate_def_with_ty { ($ (# [$ attr : meta]) * $ vis : vis $ name : ident $ (;) ?) => { $ (# [$ attr]) * # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] $ vis struct $ name (pub DefId) ; impl CrateDef for $ name { fn def_id (& self) -> DefId { self . 0 } } impl CrateDefType for $ name { } } ; }
};
}

// Generated macro for crate_def (macro)
macro_rules! Depcrate_crate_defcrate_def {
() => {
// Module: crate::crate_def
// Provides: {"crate_def"}
// Dependencies: {}
macro_rules ! crate_def { ($ (# [$ attr : meta]) * $ vis : vis $ name : ident $ (;) ?) => { $ (# [$ attr]) * # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] $ vis struct $ name (pub DefId) ; impl CrateDef for $ name { fn def_id (& self) -> DefId { self . 0 } } } ; }
};
}

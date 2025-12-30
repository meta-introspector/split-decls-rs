// Generated macro for DEFINE_GUID (macro)
macro_rules! Depcrate_macrosDEFINE_GUID {
() => {
// Module: crate::macros
// Provides: {"DEFINE_GUID"}
// Dependencies: {}
# [macro_export] macro_rules ! DEFINE_GUID { ($ name : ident , $ l : expr , $ w1 : expr , $ w2 : expr , $ b1 : expr , $ b2 : expr , $ b3 : expr , $ b4 : expr , $ b5 : expr , $ b6 : expr , $ b7 : expr , $ b8 : expr) => { pub const $ name : $ crate :: shared :: guiddef :: GUID = $ crate :: shared :: guiddef :: GUID { Data1 : $ l , Data2 : $ w1 , Data3 : $ w2 , Data4 : [$ b1 , $ b2 , $ b3 , $ b4 , $ b5 , $ b6 , $ b7 , $ b8] , } ; } }
};
}

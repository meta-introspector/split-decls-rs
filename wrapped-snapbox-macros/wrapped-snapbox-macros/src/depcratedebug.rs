// Generated macro for debug (macro)
macro_rules! Depcratedebug {
() => {
// Module: crate
// Provides: {"debug"}
// Dependencies: {}
# [doc = " Feature-flag controlled additional test debug information"] # [cfg (not (feature = "debug"))] # [macro_export] macro_rules ! debug { ($ ($ arg : tt) *) => { } ; }
};
}

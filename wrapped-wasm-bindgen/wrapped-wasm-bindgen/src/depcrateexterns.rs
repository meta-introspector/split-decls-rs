// Generated macro for externs (macro)
macro_rules! Depcrateexterns {
() => {
// Module: crate
// Provides: {"externs"}
// Dependencies: {}
macro_rules ! externs { ($ (# [$ attr : meta]) * extern "C" { $ (fn $ name : ident ($ ($ args : tt) *) -> $ ret : ty ;) * }) => (# [cfg (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")))] $ (# [$ attr]) * extern "C" { $ (fn $ name ($ ($ args) *) -> $ ret ;) * } $ (# [cfg (not (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none"))))] # [allow (unused_variables)] unsafe extern "C" fn $ name ($ ($ args) *) -> $ ret { panic ! ("function not implemented on non-wasm32 targets") }) *) }
};
}

// Generated macro for tys (macro)
macro_rules! Depcrate_tystys {
() => {
// Module: crate::tys
// Provides: {"tys"}
// Dependencies: {}
macro_rules ! tys { ($ ($ a : ident) *) => (tys ! { @ ($ ($ a) *) 0 }) ; (@ () $ v : expr) => { } ; (@ ($ a : ident $ ($ b : ident) *) $ v : expr) => { pub const $ a : u32 = $ v ; tys ! (@ ($ ($ b) *) $ v + 1) ; } }
};
}

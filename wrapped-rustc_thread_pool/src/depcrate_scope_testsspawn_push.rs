// Generated macro for spawn_push (macro)
macro_rules! Depcrate_scope_testsspawn_push {
() => {
// Module: crate::scope::tests
// Provides: {"spawn_push"}
// Dependencies: {}
macro_rules ! spawn_push { ($ scope : ident . $ spawn : ident , $ vec : ident , $ i : expr) => { { $ scope .$ spawn (move | _ | $ vec . lock () . unwrap () . push ($ i)) ; } } ; }
};
}

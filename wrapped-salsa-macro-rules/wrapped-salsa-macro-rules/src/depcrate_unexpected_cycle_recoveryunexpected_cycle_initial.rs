// Generated macro for unexpected_cycle_initial (macro)
macro_rules! Depcrate_unexpected_cycle_recoveryunexpected_cycle_initial {
() => {
// Module: crate::unexpected_cycle_recovery
// Provides: {"unexpected_cycle_initial"}
// Dependencies: {}
# [macro_export] macro_rules ! unexpected_cycle_initial { ($ db : ident , $ id : ident , $ ($ other_inputs : ident) ,*) => { { std :: mem :: drop ($ db) ; std :: mem :: drop (($ ($ other_inputs ,) *)) ; panic ! ("no cycle initial value") } } ; }
};
}

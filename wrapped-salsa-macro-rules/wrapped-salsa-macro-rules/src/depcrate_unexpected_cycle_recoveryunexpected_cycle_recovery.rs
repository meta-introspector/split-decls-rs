// Generated macro for unexpected_cycle_recovery (macro)
macro_rules! Depcrate_unexpected_cycle_recoveryunexpected_cycle_recovery {
() => {
// Module: crate::unexpected_cycle_recovery
// Provides: {"unexpected_cycle_recovery"}
// Dependencies: {}
# [macro_export] macro_rules ! unexpected_cycle_recovery { ($ db : ident , $ cycle : ident , $ last_provisional_value : ident , $ new_value : ident , $ ($ other_inputs : ident) ,*) => { { let (_db , _cycle , _last_provisional_value) = ($ db , $ cycle , $ last_provisional_value) ; std :: mem :: drop (($ ($ other_inputs ,) *)) ; $ new_value } } ; }
};
}

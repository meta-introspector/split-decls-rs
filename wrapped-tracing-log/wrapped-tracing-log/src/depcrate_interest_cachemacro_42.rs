// Generated macro for macro_42 (macro)
macro_rules! Depcrate_interest_cachemacro_42 {
() => {
// Module: crate::interest_cache
// Provides: {"macro_42"}
// Dependencies: {}
thread_local ! { static STATE : RefCell < State > = { let config = CONFIG . lock () . unwrap () ; RefCell :: new (State :: new (interest_cache_epoch () , & config)) } ; }
};
}

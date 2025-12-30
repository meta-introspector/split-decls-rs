// Generated macro for impl_num (macro)
macro_rules! Depcrateimpl_num {
() => {
// Module: crate
// Provides: {"impl_num"}
// Dependencies: {}
macro_rules ! impl_num { { $ type : tt } => { impl DefaultStrategy for $ type { type Strategy = proptest :: num ::$ type :: Any ; fn default_strategy () -> Self :: Strategy { proptest :: num ::$ type :: ANY } } } }
};
}

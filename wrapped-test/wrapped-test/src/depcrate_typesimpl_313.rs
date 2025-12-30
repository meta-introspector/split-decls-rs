// Generated macro for impl_313 (impl)
macro_rules! Depcrate_typesimpl_313 {
() => {
// Module: crate::types
// Provides: {"impl_313"}
// Dependencies: {}
impl RunnableBench { pub (crate) fn run (self , id : TestId , desc : & TestDesc , monitor_ch : & Sender < CompletedTest > , nocapture : bool ,) { match self { RunnableBench :: Static (f) => { crate :: bench :: benchmark (id , desc . clone () , monitor_ch . clone () , nocapture , f) } RunnableBench :: Dynamic (f) => { crate :: bench :: benchmark (id , desc . clone () , monitor_ch . clone () , nocapture , f) } } } }
};
}

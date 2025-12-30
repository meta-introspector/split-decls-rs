// Generated macro for impl_311 (impl)
macro_rules! Depcrate_typesimpl_311 {
() => {
// Module: crate::types
// Provides: {"impl_311"}
// Dependencies: {}
impl RunnableTest { pub (crate) fn run (self) -> Result < () , String > { match self { RunnableTest :: Static (f) => __rust_begin_short_backtrace (f) , RunnableTest :: Dynamic (f) => __rust_begin_short_backtrace (f) , RunnableTest :: StaticBenchAsTest (f) => { crate :: bench :: run_once (| b | __rust_begin_short_backtrace (| | f (b))) } RunnableTest :: DynamicBenchAsTest (f) => { crate :: bench :: run_once (| b | __rust_begin_short_backtrace (| | f (b))) } } } pub (crate) fn is_dynamic (& self) -> bool { match self { RunnableTest :: Static (_) => false , RunnableTest :: StaticBenchAsTest (_) => false , RunnableTest :: Dynamic (_) => true , RunnableTest :: DynamicBenchAsTest (_) => true , } } }
};
}

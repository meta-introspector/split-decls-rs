// Generated macro for impl_307 (impl)
macro_rules! Depcrate_typesimpl_307 {
() => {
// Module: crate::types
// Provides: {"impl_307"}
// Dependencies: {}
impl TestFn { pub fn padding (& self) -> NamePadding { match * self { StaticTestFn (..) => PadNone , StaticBenchFn (..) => PadOnRight , StaticBenchAsTestFn (..) => PadNone , DynTestFn (..) => PadNone , DynBenchFn (..) => PadOnRight , DynBenchAsTestFn (..) => PadNone , } } pub (crate) fn into_runnable (self) -> Runnable { match self { StaticTestFn (f) => Runnable :: Test (RunnableTest :: Static (f)) , StaticBenchFn (f) => Runnable :: Bench (RunnableBench :: Static (f)) , StaticBenchAsTestFn (f) => Runnable :: Test (RunnableTest :: StaticBenchAsTest (f)) , DynTestFn (f) => Runnable :: Test (RunnableTest :: Dynamic (f)) , DynBenchFn (f) => Runnable :: Bench (RunnableBench :: Dynamic (f)) , DynBenchAsTestFn (f) => Runnable :: Test (RunnableTest :: DynamicBenchAsTest (f)) , } } }
};
}

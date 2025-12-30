// Generated macro for TestFn (enum)
macro_rules! Depcrate_typesTestFn {
() => {
// Module: crate::types
// Provides: {"TestFn"}
// Dependencies: {}
pub enum TestFn { StaticTestFn (fn () -> Result < () , String >) , StaticBenchFn (fn (& mut Bencher) -> Result < () , String >) , StaticBenchAsTestFn (fn (& mut Bencher) -> Result < () , String >) , DynTestFn (Box < dyn FnOnce () -> Result < () , String > + Send >) , DynBenchFn (Box < dyn Fn (& mut Bencher) -> Result < () , String > + Send >) , DynBenchAsTestFn (Box < dyn Fn (& mut Bencher) -> Result < () , String > + Send >) , }
};
}

// Generated macro for RunnableTest (enum)
macro_rules! Depcrate_typesRunnableTest {
() => {
// Module: crate::types
// Provides: {"RunnableTest"}
// Dependencies: {}
pub (crate) enum RunnableTest { Static (fn () -> Result < () , String >) , Dynamic (Box < dyn FnOnce () -> Result < () , String > + Send >) , StaticBenchAsTest (fn (& mut Bencher) -> Result < () , String >) , DynamicBenchAsTest (Box < dyn Fn (& mut Bencher) -> Result < () , String > + Send >) , }
};
}

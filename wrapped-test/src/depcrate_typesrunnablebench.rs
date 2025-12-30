// Generated macro for RunnableBench (enum)
macro_rules! Depcrate_typesRunnableBench {
() => {
// Module: crate::types
// Provides: {"RunnableBench"}
// Dependencies: {}
pub (crate) enum RunnableBench { Static (fn (& mut Bencher) -> Result < () , String >) , Dynamic (Box < dyn Fn (& mut Bencher) -> Result < () , String > + Send >) , }
};
}

// Generated macro for CombinedTuples (type)
macro_rules! Depcrate_genericCombinedTuples {
() => {
// Module: crate::generic
// Provides: {"CombinedTuples"}
// Dependencies: {}
pub type CombinedTuples < T , U > = < < < T as Tuple > :: HList as Combine < < U as Tuple > :: HList > > :: Output as HList > :: Tuple ;
};
}

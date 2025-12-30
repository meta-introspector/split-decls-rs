// Generated macro for impl_60 (impl)
macro_rules! Depcrate_errorimpl_60 {
() => {
// Module: crate::error
// Provides: {"impl_60"}
// Dependencies: {}
impl < I : Interner > TypeError < I > { pub fn involves_regions (self) -> bool { match self { TypeError :: RegionsDoesNotOutlive (_ , _) | TypeError :: RegionsInsufficientlyPolymorphic (_ , _) | TypeError :: RegionsPlaceholderMismatch => true , _ => false , } } pub fn must_include_note (self) -> bool { use self :: TypeError :: * ; match self { CyclicTy (_) | CyclicConst (_) | SafetyMismatch (_) | PolarityMismatch (_) | Mismatch | AbiMismatch (_) | ArraySize (_) | ArgumentSorts (..) | Sorts (_) | VariadicMismatch (_) | TargetFeatureCast (_) => false , Mutability | ArgumentMutability (_) | TupleSize (_) | ArgCount | RegionsDoesNotOutlive (..) | RegionsInsufficientlyPolymorphic (..) | RegionsPlaceholderMismatch | Traits (_) | ProjectionMismatched (_) | ExistentialMismatch (_) | ConstMismatch (_) | ForceInlineCast | IntrinsicCast => true , } } }
};
}

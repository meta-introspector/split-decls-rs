// Generated macro for TypeError (enum)
macro_rules! Depcrate_errorTypeError {
() => {
// Module: crate::error
// Provides: {"TypeError"}
// Dependencies: {}
# [derive_where (Clone , Copy , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic)] # [cfg_attr (feature = "nightly" , rustc_pass_by_value)] pub enum TypeError < I : Interner > { Mismatch , PolarityMismatch (# [type_visitable (ignore)] ExpectedFound < ty :: PredicatePolarity >) , SafetyMismatch (# [type_visitable (ignore)] ExpectedFound < I :: Safety >) , AbiMismatch (# [type_visitable (ignore)] ExpectedFound < I :: Abi >) , Mutability , ArgumentMutability (usize) , TupleSize (ExpectedFound < usize >) , ArraySize (ExpectedFound < I :: Const >) , ArgCount , RegionsDoesNotOutlive (I :: Region , I :: Region) , RegionsInsufficientlyPolymorphic (I :: BoundRegion , I :: Region) , RegionsPlaceholderMismatch , Sorts (ExpectedFound < I :: Ty >) , ArgumentSorts (ExpectedFound < I :: Ty > , usize) , Traits (ExpectedFound < I :: TraitId >) , VariadicMismatch (ExpectedFound < bool >) , # [doc = " Instantiating a type variable with the given type would have"] # [doc = " created a cycle (because it appears somewhere within that"] # [doc = " type)."] CyclicTy (I :: Ty) , CyclicConst (I :: Const) , ProjectionMismatched (ExpectedFound < I :: DefId >) , ExistentialMismatch (ExpectedFound < I :: BoundExistentialPredicates >) , ConstMismatch (ExpectedFound < I :: Const >) , IntrinsicCast , # [doc = " `#[rustc_force_inline]` functions must be inlined and must not be codegened independently,"] # [doc = " so casting to a function pointer must be prohibited."] ForceInlineCast , # [doc = " Safe `#[target_feature]` functions are not assignable to safe function pointers."] TargetFeatureCast (I :: DefId) , }
};
}

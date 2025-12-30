// Generated macro for ParamEnv (trait)
macro_rules! Depcrate_inherentParamEnv {
() => {
// Module: crate::inherent
// Provides: {"ParamEnv"}
// Dependencies: {}
pub trait ParamEnv < I : Interner > : Copy + Debug + Hash + Eq + TypeFoldable < I > { fn caller_bounds (self) -> impl SliceLike < Item = I :: Clause > ; }
};
}

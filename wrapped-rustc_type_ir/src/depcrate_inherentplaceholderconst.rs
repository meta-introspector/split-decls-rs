// Generated macro for PlaceholderConst (trait)
macro_rules! Depcrate_inherentPlaceholderConst {
() => {
// Module: crate::inherent
// Provides: {"PlaceholderConst"}
// Dependencies: {}
pub trait PlaceholderConst < I : Interner > : PlaceholderLike < I , Bound = I :: BoundConst > { fn find_const_ty_from_env (self , env : I :: ParamEnv) -> I :: Ty ; }
};
}

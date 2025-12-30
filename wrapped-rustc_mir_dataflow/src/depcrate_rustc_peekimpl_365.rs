// Generated macro for impl_365 (impl)
macro_rules! Depcrate_rustc_peekimpl_365 {
() => {
// Module: crate::rustc_peek
// Provides: {"impl_365"}
// Dependencies: {}
impl PeekCallKind { fn from_arg_ty (arg : Ty < '_ >) -> Self { match arg . kind () { ty :: Ref (_ , _ , _) => PeekCallKind :: ByRef , _ => PeekCallKind :: ByVal , } } }
};
}

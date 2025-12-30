// Generated macro for impl_273 (impl)
macro_rules! Depcrate_solveimpl_273 {
() => {
// Module: crate::solve
// Provides: {"impl_273"}
// Dependencies: {}
impl < I : Interner , P > Goal < I , P > { pub fn new (cx : I , param_env : I :: ParamEnv , predicate : impl Upcast < I , P >) -> Goal < I , P > { Goal { param_env , predicate : predicate . upcast (cx) } } # [doc = " Updates the goal to one with a different `predicate` but the same `param_env`."] pub fn with < Q > (self , cx : I , predicate : impl Upcast < I , Q >) -> Goal < I , Q > { Goal { param_env : self . param_env , predicate : predicate . upcast (cx) } } }
};
}

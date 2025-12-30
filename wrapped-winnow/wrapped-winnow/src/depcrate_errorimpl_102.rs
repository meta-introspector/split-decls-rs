// Generated macro for impl_102 (impl)
macro_rules! Depcrate_errorimpl_102 {
() => {
// Module: crate::error
// Provides: {"impl_102"}
// Dependencies: {}
impl < C , I : Stream > AddContext < I , C > for ContextError < C > { # [inline] fn add_context (mut self , _input : & I , _token_start : & < I as Stream > :: Checkpoint , context : C ,) -> Self { self . push (context) ; self } }
};
}

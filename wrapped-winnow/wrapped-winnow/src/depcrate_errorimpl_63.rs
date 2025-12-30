// Generated macro for impl_63 (impl)
macro_rules! Depcrate_errorimpl_63 {
() => {
// Module: crate::error
// Provides: {"impl_63"}
// Dependencies: {}
impl < I : Stream , C , E : AddContext < I , C > > AddContext < I , C > for ErrMode < E > { # [inline (always)] fn add_context (self , input : & I , token_start : & < I as Stream > :: Checkpoint , context : C) -> Self { self . map (| err | err . add_context (input , token_start , context)) } }
};
}

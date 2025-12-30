// Generated macro for impl_114 (impl)
macro_rules! Depcrate_buffer_serviceimpl_114 {
() => {
// Module: crate::buffer::service
// Provides: {"impl_114"}
// Dependencies: {}
impl < Req , F > Clone for Buffer < Req , F > where Req : Send + 'static , F : Send + 'static , { fn clone (& self) -> Self { Self { handle : self . handle . clone () , tx : self . tx . clone () , } } }
};
}

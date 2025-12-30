// Generated macro for impl_95 (impl)
macro_rules! Depcrate_buffer_layerimpl_95 {
() => {
// Module: crate::buffer::layer
// Provides: {"impl_95"}
// Dependencies: {}
impl < S , Request > Layer < S > for BufferLayer < Request > where S : Service < Request > + Send + 'static , S :: Future : Send , S :: Error : Into < crate :: BoxError > + Send + Sync , Request : Send + 'static , { type Service = Buffer < Request , S :: Future > ; fn layer (& self , service : S) -> Self :: Service { Buffer :: new (service , self . bound) } }
};
}

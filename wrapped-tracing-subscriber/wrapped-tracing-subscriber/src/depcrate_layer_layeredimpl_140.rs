// Generated macro for impl_140 (impl)
macro_rules! Depcrate_layer_layeredimpl_140 {
() => {
// Module: crate::layer::layered
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a , L , S > LookupSpan < 'a > for Layered < L , S > where S : Subscriber + LookupSpan < 'a > , { type Data = S :: Data ; fn span_data (& 'a self , id : & span :: Id) -> Option < Self :: Data > { self . inner . span_data (id) } # [cfg (all (feature = "registry" , feature = "std"))] fn register_filter (& mut self) -> FilterId { self . inner . register_filter () } }
};
}

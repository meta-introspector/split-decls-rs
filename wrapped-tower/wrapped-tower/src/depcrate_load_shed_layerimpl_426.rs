// Generated macro for impl_426 (impl)
macro_rules! Depcrate_load_shed_layerimpl_426 {
() => {
// Module: crate::load_shed::layer
// Provides: {"impl_426"}
// Dependencies: {}
impl < S > Layer < S > for LoadShedLayer { type Service = LoadShed < S > ; fn layer (& self , service : S) -> Self :: Service { LoadShed :: new (service) } }
};
}

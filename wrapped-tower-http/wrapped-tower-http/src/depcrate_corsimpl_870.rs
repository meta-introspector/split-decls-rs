// Generated macro for impl_870 (impl)
macro_rules! Depcrate_corsimpl_870 {
() => {
// Module: crate::cors
// Provides: {"impl_870"}
// Dependencies: {}
impl < S > Layer < S > for CorsLayer { type Service = Cors < S > ; fn layer (& self , inner : S) -> Self :: Service { ensure_usable_cors_rules (self) ; Cors { inner , layer : self . clone () , } } }
};
}

// Generated macro for impl_574 (impl)
macro_rules! Depcrate_retry_layerimpl_574 {
() => {
// Module: crate::retry::layer
// Provides: {"impl_574"}
// Dependencies: {}
impl < P , S > Layer < S > for RetryLayer < P > where P : Clone , { type Service = Retry < P , S > ; fn layer (& self , service : S) -> Self :: Service { let policy = self . policy . clone () ; Retry :: new (policy , service) } }
};
}

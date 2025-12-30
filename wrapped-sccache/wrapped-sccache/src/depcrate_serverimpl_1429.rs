// Generated macro for impl_1429 (impl)
macro_rules! Depcrate_serverimpl_1429 {
() => {
// Module: crate::server
// Provides: {"impl_1429"}
// Dependencies: {}
impl < R , B > Message < R , B > { fn into_inner (self) -> R { match self { Message :: WithBody (r , _) => r , Message :: WithoutBody (r) => r , } } }
};
}

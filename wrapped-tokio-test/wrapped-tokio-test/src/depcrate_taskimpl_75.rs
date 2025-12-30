// Generated macro for impl_75 (impl)
macro_rules! Depcrate_taskimpl_75 {
() => {
// Module: crate::task
// Provides: {"impl_75"}
// Dependencies: {}
impl < T : Future > Future for Spawn < T > { type Output = T :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . future . as_mut () . poll (cx) } }
};
}

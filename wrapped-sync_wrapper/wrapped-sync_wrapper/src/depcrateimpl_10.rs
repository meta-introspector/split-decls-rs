// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < F : Future > Future for SyncFuture < F > { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let inner = unsafe { self . map_unchecked_mut (| x | x . inner . get_mut ()) } ; inner . poll (cx) } }
};
}

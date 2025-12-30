// Generated macro for impl_126 (impl)
macro_rules! Depcrate_sync_reusable_boximpl_126 {
() => {
// Module: crate::sync::reusable_box
// Provides: {"impl_126"}
// Dependencies: {}
impl < T > Future for ReusableBoxFuture < '_ , T > { type Output = T ; # [doc = " Poll the future stored inside this box."] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { Pin :: into_inner (self) . get_pin () . poll (cx) } }
};
}

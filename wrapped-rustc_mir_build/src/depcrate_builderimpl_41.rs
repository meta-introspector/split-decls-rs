// Generated macro for impl_41 (impl)
macro_rules! Depcrate_builderimpl_41 {
() => {
// Module: crate::builder
// Provides: {"impl_41"}
// Dependencies: {}
impl LocalsForNode { fn local_id (& self , for_guard : ForGuard) -> Local { match (self , for_guard) { (& LocalsForNode :: One (local_id) , ForGuard :: OutsideGuard) | (& LocalsForNode :: ForGuard { ref_for_guard : local_id , .. } , ForGuard :: RefWithinGuard ,) | (& LocalsForNode :: ForGuard { for_arm_body : local_id , .. } , ForGuard :: OutsideGuard) => { local_id } (& LocalsForNode :: One (_) , ForGuard :: RefWithinGuard) => { bug ! ("anything with one local should never be within a guard.") } } } }
};
}

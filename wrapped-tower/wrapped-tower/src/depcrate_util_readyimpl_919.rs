// Generated macro for impl_919 (impl)
macro_rules! Depcrate_util_readyimpl_919 {
() => {
// Module: crate::util::ready
// Provides: {"impl_919"}
// Dependencies: {}
impl < 'a , T , Request > Future for Ready < 'a , T , Request > where T : Service < Request > , { type Output = Result < & 'a mut T , T :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . 0) . poll (cx) } }
};
}

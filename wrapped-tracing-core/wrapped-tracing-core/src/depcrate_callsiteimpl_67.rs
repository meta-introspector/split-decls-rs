// Generated macro for impl_67 (impl)
macro_rules! Depcrate_callsiteimpl_67 {
() => {
// Module: crate::callsite
// Provides: {"impl_67"}
// Dependencies: {}
impl Callsite for DefaultCallsite { fn set_interest (& self , interest : Interest) { let interest = match () { _ if interest . is_never () => Self :: INTEREST_NEVER , _ if interest . is_always () => Self :: INTEREST_ALWAYS , _ => Self :: INTEREST_SOMETIMES , } ; self . interest . store (interest , Ordering :: SeqCst) ; } # [inline (always)] fn metadata (& self) -> & Metadata < 'static > { self . meta } }
};
}

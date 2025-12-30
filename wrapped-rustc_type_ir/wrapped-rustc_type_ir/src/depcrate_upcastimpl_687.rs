// Generated macro for impl_687 (impl)
macro_rules! Depcrate_upcastimpl_687 {
() => {
// Module: crate::upcast
// Provides: {"impl_687"}
// Dependencies: {}
impl < I , T , U > Upcast < I , U > for T where U : UpcastFrom < I , T > , { fn upcast (self , interner : I) -> U { U :: upcast_from (self , interner) } }
};
}

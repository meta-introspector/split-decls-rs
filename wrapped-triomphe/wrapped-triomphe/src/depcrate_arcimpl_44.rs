// Generated macro for impl_44 (impl)
macro_rules! Depcrate_arcimpl_44 {
() => {
// Module: crate::arc
// Provides: {"impl_44"}
// Dependencies: {}
impl < T : ? Sized > Deref for Arc < T > { type Target = T ; # [inline] fn deref (& self) -> & T { & self . inner () . data } }
};
}

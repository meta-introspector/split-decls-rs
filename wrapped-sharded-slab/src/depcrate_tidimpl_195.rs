// Generated macro for impl_195 (impl)
macro_rules! Depcrate_tidimpl_195 {
() => {
// Module: crate::tid
// Provides: {"impl_195"}
// Dependencies: {}
impl < C : cfg :: Config > Tid < C > { # [inline] pub (crate) fn current () -> Self { REGISTRATION . try_with (Registration :: current) . unwrap_or_else (| _ | Self :: poisoned ()) } pub (crate) fn is_current (self) -> bool { REGISTRATION . try_with (| r | self == r . current :: < C > ()) . unwrap_or (false) } # [inline (always)] pub fn new (id : usize) -> Self { Self :: from_usize (id) } }
};
}

// Generated macro for impl_23 (impl)
macro_rules! Depcrate_displayimpl_23 {
() => {
// Module: crate::display
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'a , T > AsDisplay < 'a > for & T where T : Display + ? Sized + 'a , { type Target = & 'a T ; fn as_display (& 'a self) -> Self :: Target { * self } }
};
}

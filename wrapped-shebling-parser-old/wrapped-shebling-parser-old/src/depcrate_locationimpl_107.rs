// Generated macro for impl_107 (impl)
macro_rules! Depcrate_locationimpl_107 {
() => {
// Module: crate::location
// Provides: {"impl_107"}
// Dependencies: {}
impl < L : Into < Location > > From < L > for Range { fn from (value : L) -> Self { let location = value . into () ; Self :: new (location , location) } }
};
}

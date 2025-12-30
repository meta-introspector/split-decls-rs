// Generated macro for impl_22 (impl)
macro_rules! Depcrate_assert_errorimpl_22 {
() => {
// Module: crate::assert::error
// Provides: {"impl_22"}
// Dependencies: {}
impl < 's > From < & 's str > for Error { fn from (other : & 's str) -> Self { Self :: with_string (other . to_owned ()) } }
};
}

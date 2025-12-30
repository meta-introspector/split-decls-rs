// Generated macro for impl_1846 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1846 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1846"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl TryFrom < HandleOrNull > for OwnedHandle { type Error = NullHandleError ; # [inline] fn try_from (handle_or_null : HandleOrNull) -> Result < Self , NullHandleError > { let handle_or_null = ManuallyDrop :: new (handle_or_null) ; if handle_or_null . is_valid () { Ok (unsafe { OwnedHandle :: from_raw_handle (handle_or_null . 0) }) } else { Err (NullHandleError (())) } } }
};
}

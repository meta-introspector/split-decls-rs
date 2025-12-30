// Generated macro for impl_1850 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1850 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1850"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl TryFrom < HandleOrInvalid > for OwnedHandle { type Error = InvalidHandleError ; # [inline] fn try_from (handle_or_invalid : HandleOrInvalid) -> Result < Self , InvalidHandleError > { let handle_or_invalid = ManuallyDrop :: new (handle_or_invalid) ; if handle_or_invalid . is_valid () { Ok (unsafe { OwnedHandle :: from_raw_handle (handle_or_invalid . 0) }) } else { Err (InvalidHandleError (())) } } }
};
}

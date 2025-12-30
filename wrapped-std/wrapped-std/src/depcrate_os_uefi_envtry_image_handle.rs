// Generated macro for try_image_handle (function)
macro_rules! Depcrate_os_uefi_envtry_image_handle {
() => {
// Module: crate::os::uefi::env
// Provides: {"try_image_handle"}
// Dependencies: {}
# [doc = " Gets the SystemHandle Pointer."] # [doc = ""] # [doc = " This function is mostly intended for places where panicking is not an option."] pub (crate) fn try_image_handle () -> Option < NonNull < c_void > > { NonNull :: new (IMAGE_HANDLE . load (Ordering :: Acquire)) }
};
}

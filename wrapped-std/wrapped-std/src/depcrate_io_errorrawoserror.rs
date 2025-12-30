// Generated macro for RawOsError (type)
macro_rules! Depcrate_io_errorRawOsError {
() => {
// Module: crate::io::error
// Provides: {"RawOsError"}
// Dependencies: {}
# [doc = " The type of raw OS error codes returned by [`Error::raw_os_error`]."] # [doc = ""] # [doc = " This is an [`i32`] on all currently supported platforms, but platforms"] # [doc = " added in the future (such as UEFI) may use a different primitive type like"] # [doc = " [`usize`]. Use `as`or [`into`] conversions where applicable to ensure maximum"] # [doc = " portability."] # [doc = ""] # [doc = " [`into`]: Into::into"] # [unstable (feature = "raw_os_error_ty" , issue = "107792")] pub type RawOsError = sys :: RawOsError ;
};
}

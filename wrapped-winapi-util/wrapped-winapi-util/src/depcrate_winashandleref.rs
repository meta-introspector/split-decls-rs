// Generated macro for AsHandleRef (trait)
macro_rules! Depcrate_winAsHandleRef {
() => {
// Module: crate::win
// Provides: {"AsHandleRef"}
// Dependencies: {}
# [doc = " Construct borrowed and valid Windows handles from file-like objects."] pub trait AsHandleRef { # [doc = " A borrowed handle that wraps the raw handle of the `Self` object."] fn as_handle_ref (& self) -> HandleRef ; # [doc = " A convenience routine for extracting a `HandleRef` from `Self`, and"] # [doc = " then extracting a raw handle from the `HandleRef`."] fn as_raw (& self) -> RawHandle { self . as_handle_ref () . as_raw_handle () } }
};
}

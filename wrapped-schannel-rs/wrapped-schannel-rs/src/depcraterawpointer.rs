// Generated macro for RawPointer (trait)
macro_rules! DepcrateRawPointer {
() => {
// Module: crate
// Provides: {"RawPointer"}
// Dependencies: {}
# [doc = " Allows access to the underlying schannel API representation of a wrapped data type"] # [doc = ""] # [doc = " Performing actions with internal handles might lead to the violation of internal assumptions"] # [doc = " and therefore is inherently unsafe."] pub trait RawPointer { # [doc = " Constructs an instance of this type from its handle / pointer."] # [doc = " # Safety"] # [doc = " This function is unsafe"] unsafe fn from_ptr (t : * mut :: std :: os :: raw :: c_void) -> Self ; # [doc = " Get a raw pointer from the underlying handle / pointer."] # [doc = " # Safety"] # [doc = " This function is unsafe"] unsafe fn as_ptr (& self) -> * mut :: std :: os :: raw :: c_void ; }
};
}

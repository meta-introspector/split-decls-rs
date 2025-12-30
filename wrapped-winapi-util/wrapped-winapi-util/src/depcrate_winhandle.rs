// Generated macro for Handle (struct)
macro_rules! Depcrate_winHandle {
() => {
// Module: crate::win
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " A handle represents an owned and valid Windows handle to a file-like"] # [doc = " object."] # [doc = ""] # [doc = " When an owned handle is dropped, then the underlying raw handle is closed."] # [doc = " To get a borrowed handle, use `HandleRef`."] # [derive (Debug)] pub struct Handle (File) ;
};
}

// Generated macro for HandleRef (struct)
macro_rules! Depcrate_winHandleRef {
() => {
// Module: crate::win
// Provides: {"HandleRef"}
// Dependencies: {}
# [doc = " Represents a borrowed and valid Windows handle to a file-like object, such"] # [doc = " as stdin/stdout/stderr or an actual file."] # [doc = ""] # [doc = " When a borrowed handle is dropped, then the underlying raw handle is"] # [doc = " **not** closed. To get an owned handle, use `Handle`."] # [derive (Debug)] pub struct HandleRef (HandleRefInner) ;
};
}

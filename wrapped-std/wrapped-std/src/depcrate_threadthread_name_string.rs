// Generated macro for thread_name_string (module)
macro_rules! Depcrate_threadthread_name_string {
() => {
// Module: crate::thread
// Provides: {"thread_name_string"}
// Dependencies: {}
mod thread_name_string { use crate :: ffi :: { CStr , CString } ; use crate :: str ; # [doc = " Like a `String` it's guaranteed UTF-8 and like a `CString` it's null terminated."] pub (crate) struct ThreadNameString { inner : CString , } impl From < String > for ThreadNameString { fn from (s : String) -> Self { Self { inner : CString :: new (s) . expect ("thread name may not contain interior null bytes") , } } } impl ThreadNameString { pub fn as_cstr (& self) -> & CStr { & self . inner } pub fn as_str (& self) -> & str { unsafe { str :: from_utf8_unchecked (self . inner . to_bytes ()) } } } }
};
}

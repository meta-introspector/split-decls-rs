// Generated macro for impl_52 (impl)
macro_rules! Depcrate_requestimpl_52 {
() => {
// Module: crate::request
// Provides: {"impl_52"}
// Dependencies: {}
impl Index < & str > for Request { type Output = HeaderValues ; # [doc = " Returns a reference to the value corresponding to the supplied name."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the name is not present in `Request`."] # [inline] fn index (& self , name : & str) -> & HeaderValues { & self . req [name] } }
};
}

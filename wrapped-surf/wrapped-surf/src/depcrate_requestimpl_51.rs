// Generated macro for impl_51 (impl)
macro_rules! Depcrate_requestimpl_51 {
() => {
// Module: crate::request
// Provides: {"impl_51"}
// Dependencies: {}
impl Index < HeaderName > for Request { type Output = HeaderValues ; # [doc = " Returns a reference to the value corresponding to the supplied name."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the name is not present in `Request`."] # [inline] fn index (& self , name : HeaderName) -> & HeaderValues { & self . req [name] } }
};
}

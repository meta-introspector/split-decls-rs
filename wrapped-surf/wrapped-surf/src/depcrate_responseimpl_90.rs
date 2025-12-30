// Generated macro for impl_90 (impl)
macro_rules! Depcrate_responseimpl_90 {
() => {
// Module: crate::response
// Provides: {"impl_90"}
// Dependencies: {}
impl Index < & str > for Response { type Output = HeaderValues ; # [doc = " Returns a reference to the value corresponding to the supplied name."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the name is not present in `Response`."] # [inline] fn index (& self , name : & str) -> & HeaderValues { & self . res [name] } }
};
}

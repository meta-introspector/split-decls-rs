// Generated macro for impl_89 (impl)
macro_rules! Depcrate_responseimpl_89 {
() => {
// Module: crate::response
// Provides: {"impl_89"}
// Dependencies: {}
impl Index < HeaderName > for Response { type Output = HeaderValues ; # [doc = " Returns a reference to the value corresponding to the supplied name."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the name is not present in `Response`."] # [inline] fn index (& self , name : HeaderName) -> & HeaderValues { & self . res [name] } }
};
}

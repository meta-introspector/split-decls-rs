// Generated macro for impl_310 (impl)
macro_rules! Depcrateimpl_310 {
() => {
// Module: crate
// Provides: {"impl_310"}
// Dependencies: {}
impl From < i32 > for ReturnCode { fn from (value : i32) -> Self { match Self :: try_from_c_int (value) { Some (value) => value , None => panic ! ("invalid return code {value}") , } } }
};
}

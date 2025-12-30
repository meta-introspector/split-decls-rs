// Generated macro for impl_139 (impl)
macro_rules! Depcrate_deflateimpl_139 {
() => {
// Module: crate::deflate
// Provides: {"impl_139"}
// Dependencies: {}
impl TryFrom < i32 > for Method { type Error = () ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { 8 => Ok (Self :: Deflated) , _ => Err (()) , } } }
};
}

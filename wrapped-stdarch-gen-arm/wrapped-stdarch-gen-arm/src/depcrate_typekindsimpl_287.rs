// Generated macro for impl_287 (impl)
macro_rules! Depcrate_typekindsimpl_287 {
() => {
// Module: crate::typekinds
// Provides: {"impl_287"}
// Dependencies: {}
impl TryFrom < usize > for VectorTupleSize { type Error = String ; fn try_from (value : usize) -> Result < Self , Self :: Error > { match value { 2 => Ok (Self :: Two) , 3 => Ok (Self :: Three) , 4 => Ok (Self :: Four) , _ => Err (format ! ("invalid vector tuple size `{value}` provided")) , } } }
};
}

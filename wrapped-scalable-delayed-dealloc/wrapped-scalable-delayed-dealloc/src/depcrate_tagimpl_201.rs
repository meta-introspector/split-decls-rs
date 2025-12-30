// Generated macro for impl_201 (impl)
macro_rules! Depcrate_tagimpl_201 {
() => {
// Module: crate::tag
// Provides: {"impl_201"}
// Dependencies: {}
impl TryFrom < u8 > for Tag { type Error = u8 ; # [inline] fn try_from (val : u8) -> Result < Self , Self :: Error > { match val { 0 => Ok (Tag :: None) , 1 => Ok (Tag :: First) , 2 => Ok (Tag :: Second) , 3 => Ok (Tag :: Both) , _ => Err (val) , } } }
};
}

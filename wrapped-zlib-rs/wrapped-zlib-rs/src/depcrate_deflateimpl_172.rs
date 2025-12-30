// Generated macro for impl_172 (impl)
macro_rules! Depcrate_deflateimpl_172 {
() => {
// Module: crate::deflate
// Provides: {"impl_172"}
// Dependencies: {}
impl TryFrom < i32 > for Strategy { type Error = () ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { 0 => Ok (Strategy :: Default) , 1 => Ok (Strategy :: Filtered) , 2 => Ok (Strategy :: HuffmanOnly) , 3 => Ok (Strategy :: Rle) , 4 => Ok (Strategy :: Fixed) , _ => Err (()) , } } }
};
}

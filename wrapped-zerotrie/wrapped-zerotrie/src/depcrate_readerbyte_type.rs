// Generated macro for byte_type (function)
macro_rules! Depcrate_readerbyte_type {
() => {
// Module: crate::reader
// Provides: {"byte_type"}
// Dependencies: {}
# [inline] fn byte_type (b : u8) -> NodeType { match b & 0b11100000 { 0b10000000 => NodeType :: Value , 0b10100000 => NodeType :: Span , 0b11000000 => NodeType :: Branch , 0b11100000 => NodeType :: Branch , _ => NodeType :: Ascii , } }
};
}

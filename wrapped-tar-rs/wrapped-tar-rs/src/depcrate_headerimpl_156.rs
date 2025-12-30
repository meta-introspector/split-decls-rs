// Generated macro for impl_156 (impl)
macro_rules! Depcrate_headerimpl_156 {
() => {
// Module: crate::header
// Provides: {"impl_156"}
// Dependencies: {}
impl GnuSparseHeader { # [doc = " Returns true if block is empty"] pub fn is_empty (& self) -> bool { self . offset [0] == 0 || self . numbytes [0] == 0 } # [doc = " Offset of the block from the start of the file"] # [doc = ""] # [doc = " Returns `Err` for a malformed `offset` field."] pub fn offset (& self) -> io :: Result < u64 > { num_field_wrapper_from (& self . offset) . map_err (| err | { io :: Error :: new (err . kind () , format ! ("{} when getting offset from sparse header" , err) ,) }) } # [doc = " Encodes the `offset` provided into this header."] pub fn set_offset (& mut self , offset : u64) { num_field_wrapper_into (& mut self . offset , offset) ; } # [doc = " Length of the block"] # [doc = ""] # [doc = " Returns `Err` for a malformed `numbytes` field."] pub fn length (& self) -> io :: Result < u64 > { num_field_wrapper_from (& self . numbytes) . map_err (| err | { io :: Error :: new (err . kind () , format ! ("{} when getting length from sparse header" , err) ,) }) } # [doc = " Encodes the `length` provided into this header."] pub fn set_length (& mut self , length : u64) { num_field_wrapper_into (& mut self . numbytes , length) ; } }
};
}

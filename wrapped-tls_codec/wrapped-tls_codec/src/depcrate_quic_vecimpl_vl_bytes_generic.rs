// Generated macro for impl_vl_bytes_generic (macro)
macro_rules! Depcrate_quic_vecimpl_vl_bytes_generic {
() => {
// Module: crate::quic_vec
// Provides: {"impl_vl_bytes_generic"}
// Dependencies: {}
macro_rules ! impl_vl_bytes_generic { ($ name : ident) => { impl fmt :: Debug for $ name { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { write ! (f , "{} {{ " , stringify ! ($ name)) ?; write_hex (f , & self . vec ()) ?; write ! (f , " }}") } } impl $ name { # [doc = " Get a reference to the vlbytes's vec."] pub fn as_slice (& self) -> & [u8] { self . vec () . as_ref () } # [doc = " Add an element to this."] # [inline] pub fn push (& mut self , value : u8) { self . vec_mut () . push (value) ; } # [doc = " Remove the last element."] # [inline] pub fn pop (& mut self) -> Option < u8 > { self . vec_mut () . pop () } } impl From < Vec < u8 >> for $ name { fn from (vec : Vec < u8 >) -> Self { Self :: new (vec) } } impl From <& [u8] > for $ name { fn from (slice : & [u8]) -> Self { Self :: new (slice . to_vec ()) } } impl < const N : usize > From <& [u8 ; N] > for $ name { fn from (slice : & [u8 ; N]) -> Self { Self :: new (slice . to_vec ()) } } impl AsRef < [u8] > for $ name { fn as_ref (& self) -> & [u8] { & self . vec () } } } ; }
};
}

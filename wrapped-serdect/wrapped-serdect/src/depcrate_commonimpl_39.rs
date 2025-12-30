// Generated macro for impl_39 (impl)
macro_rules! Depcrate_commonimpl_39 {
() => {
// Module: crate::common
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'b , T : LengthCheck > Visitor < '_ > for SliceVisitor < 'b , T > { type Value = & 'b [u8] ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { T :: expecting (formatter , "an array" , self . 0 . len ()) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { if T :: length_check (self . 0 . len () , v . len ()) { let buffer = & mut self . 0 [.. v . len ()] ; buffer . copy_from_slice (v) ; return Ok (buffer) ; } Err (E :: invalid_length (v . len () , & self)) } # [cfg (feature = "alloc")] fn visit_byte_buf < E > (self , mut v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { if T :: length_check (self . 0 . len () , v . len ()) { let buffer = & mut self . 0 [.. v . len ()] ; buffer . swap_with_slice (& mut v) ; return Ok (buffer) ; } Err (E :: invalid_length (v . len () , & self)) } }
};
}

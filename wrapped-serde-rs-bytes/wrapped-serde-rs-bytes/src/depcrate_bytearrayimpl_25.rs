// Generated macro for impl_25 (impl)
macro_rules! Depcrate_bytearrayimpl_25 {
() => {
// Module: crate::bytearray
// Provides: {"impl_25"}
// Dependencies: {}
impl < Rhs , const N : usize > PartialEq < Rhs > for ByteArray < N > where Rhs : ? Sized + Borrow < [u8 ; N] > , { fn eq (& self , other : & Rhs) -> bool { self . as_ref () . eq (other . borrow ()) } }
};
}

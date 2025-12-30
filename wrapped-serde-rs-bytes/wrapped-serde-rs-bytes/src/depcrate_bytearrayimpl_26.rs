// Generated macro for impl_26 (impl)
macro_rules! Depcrate_bytearrayimpl_26 {
() => {
// Module: crate::bytearray
// Provides: {"impl_26"}
// Dependencies: {}
impl < Rhs , const N : usize > PartialOrd < Rhs > for ByteArray < N > where Rhs : ? Sized + Borrow < [u8 ; N] > , { fn partial_cmp (& self , other : & Rhs) -> Option < Ordering > { self . as_ref () . partial_cmp (other . borrow ()) } }
};
}

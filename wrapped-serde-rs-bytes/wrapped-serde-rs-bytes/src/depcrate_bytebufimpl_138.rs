// Generated macro for impl_138 (impl)
macro_rules! Depcrate_bytebufimpl_138 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_138"}
// Dependencies: {}
impl < Rhs > PartialOrd < Rhs > for ByteBuf where Rhs : ? Sized + AsRef < [u8] > , { fn partial_cmp (& self , other : & Rhs) -> Option < Ordering > { self . as_ref () . partial_cmp (other . as_ref ()) } }
};
}

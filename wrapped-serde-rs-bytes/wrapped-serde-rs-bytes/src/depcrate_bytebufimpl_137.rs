// Generated macro for impl_137 (impl)
macro_rules! Depcrate_bytebufimpl_137 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_137"}
// Dependencies: {}
impl < Rhs > PartialEq < Rhs > for ByteBuf where Rhs : ? Sized + AsRef < [u8] > , { fn eq (& self , other : & Rhs) -> bool { self . as_ref () . eq (other . as_ref ()) } }
};
}

// Generated macro for impl_10 (impl)
macro_rules! Depcrate_arraysimpl_10 {
() => {
// Module: crate::arrays
// Provides: {"impl_10"}
// Dependencies: {}
impl < const LEN : usize > Serialize for [u8 ; LEN] { # [cfg (feature = "std")] # [inline] fn tls_serialize < W : Write > (& self , writer : & mut W) -> Result < usize , Error > { let written = writer . write (self) ? ; if written == LEN { Ok (written) } else { Err (Error :: InvalidWriteLength (format ! ("Expected to write {LEN} bytes but only {written} were written."))) } } }
};
}

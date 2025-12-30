// Generated macro for impl_423 (impl)
macro_rules! Depcrate_sigabiimpl_423 {
() => {
// Module: crate::sigabi
// Provides: {"impl_423"}
// Dependencies: {}
impl SenderInfo { # [inline] pub fn raw (self) -> u64 { u64 :: from (self . pid) | (u64 :: from (self . ruid) << 32) } # [inline] pub const fn from_raw (raw : u64) -> Self { Self { pid : raw as u32 , ruid : (raw >> 32) as u32 , } } }
};
}

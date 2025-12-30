// Generated macro for Buf32 (struct)
macro_rules! Depcrate_buf32Buf32 {
() => {
// Module: crate::buf32
// Provides: {"Buf32"}
// Dependencies: {}
# [doc = " A buffer points to a header of type `H`, which is followed by `MIN_CAP` or more"] # [doc = " bytes of storage."] # [repr (packed)] pub struct Buf32 < H > { pub ptr : * mut H , pub len : u32 , pub cap : u32 , }
};
}

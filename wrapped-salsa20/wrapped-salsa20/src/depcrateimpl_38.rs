// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl < R : Unsigned > KeyIvInit for SalsaCore < R > { fn new (key : & Key , iv : & Nonce) -> Self { let mut state = [0u32 ; STATE_WORDS] ; state [0] = CONSTANTS [0] ; for (i , chunk) in key [.. 16] . chunks (4) . enumerate () { state [1 + i] = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } state [5] = CONSTANTS [1] ; for (i , chunk) in iv . chunks (4) . enumerate () { state [6 + i] = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } state [8] = 0 ; state [9] = 0 ; state [10] = CONSTANTS [2] ; for (i , chunk) in key [16 ..] . chunks (4) . enumerate () { state [11 + i] = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } state [15] = CONSTANTS [3] ; Self { state , rounds : PhantomData , } } }
};
}

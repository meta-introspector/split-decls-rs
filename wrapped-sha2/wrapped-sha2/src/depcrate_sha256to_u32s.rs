// Generated macro for to_u32s (function)
macro_rules! Depcrate_sha256to_u32s {
() => {
// Module: crate::sha256
// Provides: {"to_u32s"}
// Dependencies: {}
# [inline (always)] # [allow (dead_code)] fn to_u32s (block : & [u8 ; 64]) -> [u32 ; 16] { core :: array :: from_fn (| i | { let chunk = block [4 * i ..] [.. 4] . try_into () . unwrap () ; u32 :: from_be_bytes (chunk) }) }
};
}

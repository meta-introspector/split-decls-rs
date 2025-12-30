// Generated macro for Sha512VarCore (struct)
macro_rules! Depcrate_block_apiSha512VarCore {
() => {
// Module: crate::block_api
// Provides: {"Sha512VarCore"}
// Dependencies: {}
# [doc = " Core block-level SHA-512 hasher with variable output size."] # [doc = ""] # [doc = " Supports initialization only for 28, 32, 48, and 64 byte output sizes,"] # [doc = " i.e. 224, 256, 384, and 512 bits respectively."] # [derive (Clone)] pub struct Sha512VarCore { state : consts :: State512 , block_len : u128 , }
};
}

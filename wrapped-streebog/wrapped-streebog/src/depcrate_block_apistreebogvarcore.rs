// Generated macro for StreebogVarCore (struct)
macro_rules! Depcrate_block_apiStreebogVarCore {
() => {
// Module: crate::block_api
// Provides: {"StreebogVarCore"}
// Dependencies: {}
# [doc = " Core block-level Streebog hasher with variable output size."] # [doc = ""] # [doc = " Supports initialization only for 32 and 64 byte output sizes,"] # [doc = " i.e. 256 and 512 bits respectively."] # [derive (Clone)] pub struct StreebogVarCore { h : [u64 ; 8] , n : [u64 ; 8] , sigma : [u64 ; 8] , }
};
}

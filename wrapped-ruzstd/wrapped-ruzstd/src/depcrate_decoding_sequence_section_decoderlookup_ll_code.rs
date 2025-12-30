// Generated macro for lookup_ll_code (function)
macro_rules! Depcrate_decoding_sequence_section_decoderlookup_ll_code {
() => {
// Module: crate::decoding::sequence_section_decoder
// Provides: {"lookup_ll_code"}
// Dependencies: {}
# [doc = " Look up the provided state value from a literal length table predefined"] # [doc = " by the Zstandard reference document. Returns a tuple of (value, number of bits)."] # [doc = ""] # [doc = " <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#appendix-a---decoding-tables-for-predefined-codes>"] fn lookup_ll_code (code : u8) -> (u32 , u8) { match code { 0 ..= 15 => (u32 :: from (code) , 0) , 16 => (16 , 1) , 17 => (18 , 1) , 18 => (20 , 1) , 19 => (22 , 1) , 20 => (24 , 2) , 21 => (28 , 2) , 22 => (32 , 3) , 23 => (40 , 3) , 24 => (48 , 4) , 25 => (64 , 6) , 26 => (128 , 7) , 27 => (256 , 8) , 28 => (512 , 9) , 29 => (1024 , 10) , 30 => (2048 , 11) , 31 => (4096 , 12) , 32 => (8192 , 13) , 33 => (16384 , 14) , 34 => (32768 , 15) , 35 => (65536 , 16) , _ => unreachable ! ("Illegal literal length code was: {}" , code) , } }
};
}

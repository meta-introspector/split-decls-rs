// Generated macro for read_u64 (function)
macro_rules! Depcrate_vli_encread_u64 {
() => {
// Module: crate::vli_enc
// Provides: {"read_u64"}
// Dependencies: {}
# [doc = " Read an encoded 64 bit number from a buffered reader."] # [doc = ""] # [doc = " This uses [`read_u64_fast`] if the remaining buffer is larger than 16 bytes and falls back to a"] # [doc = " slower implementation otherwise."] pub fn read_u64 (source : & mut impl BufRead) -> Result < u64 , io :: Error > { let buf = source . fill_buf () ? ; if buf . len () >= 16 { let next_bytes : & [u8 ; 16] = (& buf [.. 16]) . try_into () . unwrap () ; let (value , advance) = read_u64_fast (next_bytes) ; source . consume (advance) ; Ok (value) } else { let mut scan = 1 << 9 ; let mut byte : [u8 ; 1] = [0] ; let mut begin = 1 ; source . read_exact (& mut byte [..]) ? ; let mut result = byte [0] as u64 ; scan |= byte [0] as u64 ; if byte [0] == 0 { begin += 1 ; source . read_exact (& mut byte [..]) ? ; result |= (byte [0] as u64) << 8 ; scan |= (byte [0] as u64) << 8 ; } let len = scan . trailing_zeros () + 1 ; result >>= len ; for i in begin .. len { source . read_exact (& mut byte [..]) ? ; result |= (byte [0] as u64) << (8 * i - len) ; } Ok (result as u64) } }
};
}

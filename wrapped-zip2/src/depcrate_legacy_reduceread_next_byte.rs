// Generated macro for read_next_byte (function)
macro_rules! Depcrate_legacy_reduceread_next_byte {
() => {
// Module: crate::legacy::reduce
// Provides: {"read_next_byte"}
// Dependencies: {}
# [doc = " Read the next byte from is, decoded based on prev_byte and the follower sets."] # [doc = " The byte is returned in *out_byte."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " * `Ok` with the byte if it was successfully read."] # [doc = " * `Err(io::Error)` on bad data or end of input."] fn read_next_byte < T : std :: io :: Read , E : Endianness > (is : & mut BitReader < T , E > , prev_byte : u8 , fsets : & mut FollowerSetArray ,) -> io :: Result < u8 > { if fsets [prev_byte as usize] . size == 0 || is . read :: < 1 , u8 > () ? == 1 { return is . read :: < 8 , u8 > () ; } let idx_bitlen = fsets [prev_byte as usize] . idx_bitlen ; let follower_idx = is . read_var :: < u16 > (idx_bitlen as u32) ? as usize ; if follower_idx >= fsets [prev_byte as usize] . size as usize { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "invalid follower index" ,)) ; } Ok (fsets [prev_byte as usize] . followers [follower_idx]) }
};
}

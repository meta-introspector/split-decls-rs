// Generated macro for write_all_bytes (function)
macro_rules! Depcrate_decoding_decode_bufferwrite_all_bytes {
() => {
// Module: crate::decoding::decode_buffer
// Provides: {"write_all_bytes"}
// Dependencies: {}
# [doc = " Like Write::write_all but returns partial write length even on error"] fn write_all_bytes (mut sink : impl Write , buf : & [u8]) -> (usize , Result < () , Error >) { let mut written = 0 ; while written < buf . len () { match sink . write (& buf [written ..]) { Ok (0) => return (written , Ok (())) , Ok (w) => written += w , Err (e) => return (written , Err (e)) , } } (written , Ok (())) }
};
}

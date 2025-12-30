// Generated macro for impl_189 (impl)
macro_rules! Depcrate_decoding_decode_bufferimpl_189 {
() => {
// Module: crate::decoding::decode_buffer
// Provides: {"impl_189"}
// Dependencies: {}
impl Read for DecodeBuffer { fn read (& mut self , target : & mut [u8]) -> Result < usize , Error > { let max_amount = self . can_drain_to_window_size () . unwrap_or (0) ; let amount = max_amount . min (target . len ()) ; let mut written = 0 ; self . drain_to (amount , | buf | { target [written ..] [.. buf . len ()] . copy_from_slice (buf) ; written += buf . len () ; (buf . len () , Ok (())) }) ? ; Ok (amount) } }
};
}

// Generated macro for inlined_slow_read_byte (function)
macro_rules! Depcrate_ioinlined_slow_read_byte {
() => {
// Module: crate::io
// Provides: {"inlined_slow_read_byte"}
// Dependencies: {}
# [doc = " Reads a single byte in a slow, generic way. This is used by the default"] # [doc = " `spec_read_byte`."] # [inline] fn inlined_slow_read_byte < R : Read > (reader : & mut R) -> Option < Result < u8 > > { let mut byte = 0 ; loop { return match reader . read (slice :: from_mut (& mut byte)) { Ok (0) => None , Ok (..) => Some (Ok (byte)) , Err (ref e) if e . is_interrupted () => continue , Err (e) => Some (Err (e)) , } ; } }
};
}

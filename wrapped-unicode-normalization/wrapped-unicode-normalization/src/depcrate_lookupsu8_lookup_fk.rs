// Generated macro for u8_lookup_fk (function)
macro_rules! Depcrate_lookupsu8_lookup_fk {
() => {
// Module: crate::lookups
// Provides: {"u8_lookup_fk"}
// Dependencies: {}
# [doc = " Extract the key in a 24 bit key and 8 bit value packed in a u32."] # [inline] fn u8_lookup_fk (kv : u32) -> u32 { kv >> 8 }
};
}

// Generated macro for pack_str (function)
macro_rules! Depcrate_writerpack_str {
() => {
// Module: crate::writer
// Provides: {"pack_str"}
// Dependencies: {}
# [doc = " Convert the given string into a u64, where the least significant byte of"] # [doc = " the u64 is the first byte of the string."] # [doc = ""] # [doc = " If the string contains any `NUL` bytes or has more than 8 bytes, then an"] # [doc = " error is returned."] fn pack_str (s : & str) -> Result < u64 > { if s . len () > 8 { return err ! ("cannot encode string {:?} (too long)" , s) ; } if s . contains ('\x00') { return err ! ("cannot encode string {:?} (contains NUL byte)" , s) ; } let mut value = 0 ; for (i , & b) in s . as_bytes () . iter () . enumerate () { assert ! (i <= 7) ; value |= (b as u64) << (8 * i as u64) ; } Ok (value) }
};
}

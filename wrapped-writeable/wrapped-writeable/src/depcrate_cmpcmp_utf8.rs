// Generated macro for cmp_utf8 (function)
macro_rules! Depcrate_cmpcmp_utf8 {
() => {
// Module: crate::cmp
// Provides: {"cmp_utf8"}
// Dependencies: {}
# [doc = " Compares the contents of a [`Writeable`] to the given UTF-8 bytes without allocating memory."] # [doc = ""] # [doc = " For more details, see: [`cmp_str`]"] pub fn cmp_utf8 (writeable : & impl Writeable , other : & [u8]) -> Ordering { let mut wc = WriteComparator :: new (other) ; let _ = writeable . write_to (& mut wc) ; wc . finish () . reverse () }
};
}

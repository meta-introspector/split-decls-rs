// Generated macro for ptr_as_str (function)
macro_rules! Depcrate_identifierptr_as_str {
() => {
// Module: crate::identifier
// Provides: {"ptr_as_str"}
// Dependencies: {}
unsafe fn ptr_as_str (repr : & NonNull < u8 >) -> & str { let ptr = repr_to_ptr (* repr) ; let len = unsafe { decode_len (ptr) } ; let header = bytes_for_varint (len) ; let slice = unsafe { slice :: from_raw_parts (ptr . add (header) , len . get ()) } ; unsafe { str :: from_utf8_unchecked (slice) } }
};
}

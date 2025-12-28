macro_rules! ptr_as_str {
    () => {
        unsafe fn ptr_as_str (repr : & NonNull < u8 >) -> & str { let ptr = repr_to_ptr (* repr) ; let len = unsafe { decode_len (ptr) } ; let header = bytes_for_varint (len) ; let slice = unsafe { slice :: from_raw_parts (ptr . add (header) , len . get ()) } ; unsafe { str :: from_utf8_unchecked (slice) } }
    };
}

ptr_as_str!()
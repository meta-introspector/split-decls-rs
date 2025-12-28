macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! inline_as_str {
    () => {
        deps!();
        unsafe fn inline_as_str (repr : & Identifier) -> & str { let ptr = repr as * const Identifier as * const u8 ; let len = unsafe { inline_len (repr) } . get () ; let slice = unsafe { slice :: from_raw_parts (ptr , len) } ; unsafe { str :: from_utf8_unchecked (slice) } }
    };
}

inline_as_str!();
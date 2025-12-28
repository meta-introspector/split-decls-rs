macro_rules! deps {
    () => {
        MultiByteChar!();
    };
}

macro_rules! analyze_source_file_generic {
    () => {
        deps!();
        fn analyze_source_file_generic (src : & str , scan_len : usize , output_offset : RelativeBytePos , lines : & mut Vec < RelativeBytePos > , multi_byte_chars : & mut Vec < MultiByteChar > ,) -> usize { assert ! (src . len () >= scan_len) ; let mut i = 0 ; let src_bytes = src . as_bytes () ; while i < scan_len { let byte = unsafe { * src_bytes . get_unchecked (i) } ; let mut char_len = 1 ; if byte == b'\n' { let pos = RelativeBytePos :: from_usize (i) + output_offset ; lines . push (pos + RelativeBytePos (1)) ; } else if byte >= 128 { let c = src [i ..] . chars () . next () . unwrap () ; char_len = c . len_utf8 () ; let pos = RelativeBytePos :: from_usize (i) + output_offset ; assert ! ((2 ..= 4) . contains (& char_len)) ; let mbc = MultiByteChar { pos , bytes : char_len as u8 } ; multi_byte_chars . push (mbc) ; } i += char_len ; } i - scan_len }
    };
}

analyze_source_file_generic!();
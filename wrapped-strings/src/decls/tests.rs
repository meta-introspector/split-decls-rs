macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: * ; # [test] fn test () { assert_eq ! (decode_utf8_char (b"123" , 0) , Some ((0x31 , 1))) ; assert_eq ! (decode_utf8_char (b"123" , 1) , Some ((0x32 , 2))) ; assert_eq ! (decode_utf8_char (b"123" , 2) , Some ((0x33 , 3))) ; assert_eq ! (decode_utf8_char (b"123" , 3) , None) ; assert_eq ! (utf16_len (b"123") , 3) ; assert_eq ! (utf16_len ("α & ω" . as_bytes ()) , 5) ; } }
    };
}

tests!()
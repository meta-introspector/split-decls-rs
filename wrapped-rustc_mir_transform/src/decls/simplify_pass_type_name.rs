macro_rules! simplify_pass_type_name {
    () => {
        const fn simplify_pass_type_name (name : & 'static str) -> & 'static str { let bytes = name . as_bytes () ; let mut i = bytes . len () ; while i > 0 && bytes [i - 1] != b':' { i -= 1 ; } let (_ , bytes) = bytes . split_at (i) ; let mut i = 0 ; while i < bytes . len () && bytes [i] != b'<' { i += 1 ; } let (bytes , _) = bytes . split_at (i) ; match std :: str :: from_utf8 (bytes) { Ok (name) => name , Err (_) => panic ! () , } }
    };
}

simplify_pass_type_name!();
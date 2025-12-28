macro_rules! syncsearch {
    () => {
        fn syncsearch (mut got : usize , buf : & [u8]) -> (usize , usize) { let len = buf . len () ; let mut next = 0 ; while next < len && got < 4 { if buf [next] == if got < 2 { 0 } else { 0xff } { got += 1 ; } else if buf [next] != 0 { got = 0 ; } else { got = 4 - got ; } next += 1 ; } (got , next) }
    };
}

syncsearch!();
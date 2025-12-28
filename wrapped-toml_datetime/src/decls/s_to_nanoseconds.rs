macro_rules! s_to_nanoseconds {
    () => {
        fn s_to_nanoseconds (input : & str) -> u32 { let mut nanosecond = 0 ; for (i , byte) in input . bytes () . enumerate () { if byte . is_ascii_digit () { if i < 9 { let p = 10_u32 . pow (8 - i as u32) ; nanosecond += p * u32 :: from (byte - b'0') ; } } else { panic ! ("invalid nanoseconds {input:?}") ; } } nanosecond }
    };
}

s_to_nanoseconds!()
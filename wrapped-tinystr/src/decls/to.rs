macro_rules! deps {
    () => {
        Aligned8!();
        Aligned4!();
        AsciiByte!();
    };
}

macro_rules! to {
    () => {
        deps!();
        macro_rules ! to { ($ self : ident , $ to : ident , $ later_char_to : ident $ (,$ first_char_to : ident) ?) => { { let mut i = 0 ; if N <= 4 { let aligned = Aligned4 :: from_ascii_bytes (&$ self . bytes) .$ to () . to_ascii_bytes () ; # [expect (clippy :: indexing_slicing)] while i < N { $ self . bytes [i] = aligned [i] ; i += 1 ; } } else if N <= 8 { let aligned = Aligned8 :: from_ascii_bytes (&$ self . bytes) .$ to () . to_ascii_bytes () ; # [expect (clippy :: indexing_slicing)] while i < N { $ self . bytes [i] = aligned [i] ; i += 1 ; } } else { while i < N && $ self . bytes [i] as u8 != AsciiByte :: B0 as u8 { unsafe { $ self . bytes [i] = core :: mem :: transmute ::< u8 , AsciiByte > (($ self . bytes [i] as u8) .$ later_char_to ()) ; } i += 1 ; } $ ($ self . bytes [0] = unsafe { core :: mem :: transmute ::< u8 , AsciiByte > (($ self . bytes [0] as u8) .$ first_char_to ()) } ;) ? } $ self } } ; }
    };
}

to!();
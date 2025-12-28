macro_rules! deps {
    () => {
        Hyphenated!();
    };
}

macro_rules! encode_hyphenated {
    () => {
        deps!();
        # [inline] fn encode_hyphenated < 'b > (src : & [u8 ; 16] , buffer : & 'b mut [u8] , upper : bool) -> & 'b mut str { let buf = & mut buffer [.. Hyphenated :: LENGTH] ; let buf : & mut [u8 ; Hyphenated :: LENGTH] = buf . try_into () . unwrap () ; * buf = format_hyphenated (src , upper) ; unsafe { str :: from_utf8_unchecked_mut (buf) } }
    };
}

encode_hyphenated!()
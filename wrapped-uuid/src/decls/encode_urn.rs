macro_rules! deps {
    () => {
        Hyphenated!();
        Urn!();
    };
}

macro_rules! encode_urn {
    () => {
        deps!();
        # [inline] fn encode_urn < 'b > (src : & [u8 ; 16] , buffer : & 'b mut [u8] , upper : bool) -> & 'b mut str { let buf = & mut buffer [.. Urn :: LENGTH] ; buf [.. 9] . copy_from_slice (b"urn:uuid:") ; let dst = & mut buf [9 .. (9 + Hyphenated :: LENGTH)] ; let dst : & mut [u8 ; Hyphenated :: LENGTH] = dst . try_into () . unwrap () ; * dst = format_hyphenated (src , upper) ; unsafe { str :: from_utf8_unchecked_mut (buf) } }
    };
}

encode_urn!();
macro_rules! deps {
    () => {
        Simple!();
    };
}

macro_rules! encode_simple {
    () => {
        deps!();
        # [inline] fn encode_simple < 'b > (src : & [u8 ; 16] , buffer : & 'b mut [u8] , upper : bool) -> & 'b mut str { let buf = & mut buffer [.. Simple :: LENGTH] ; let buf : & mut [u8 ; Simple :: LENGTH] = buf . try_into () . unwrap () ; * buf = format_simple (src , upper) ; unsafe { str :: from_utf8_unchecked_mut (buf) } }
    };
}

encode_simple!()
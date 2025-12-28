macro_rules! deps {
    () => {
        Hyphenated!();
        Braced!();
    };
}

macro_rules! encode_braced {
    () => {
        deps!();
        # [inline] fn encode_braced < 'b > (src : & [u8 ; 16] , buffer : & 'b mut [u8] , upper : bool) -> & 'b mut str { let buf = & mut buffer [.. Hyphenated :: LENGTH + 2] ; let buf : & mut [u8 ; Hyphenated :: LENGTH + 2] = buf . try_into () . unwrap () ; # [cfg_attr (all (uuid_unstable , feature = "zerocopy") , derive (zerocopy :: IntoBytes))] # [repr (C)] struct Braced { open_curly : u8 , hyphenated : [u8 ; Hyphenated :: LENGTH] , close_curly : u8 , } let braced = Braced { open_curly : b'{' , hyphenated : format_hyphenated (src , upper) , close_curly : b'}' , } ; * buf = unsafe_transmute ! (braced) ; unsafe { str :: from_utf8_unchecked_mut (buf) } }
    };
}

encode_braced!()
macro_rules! deps {
    () => {
        HSTRING!();
        PCWSTR!();
    };
}

macro_rules! h {
    () => {
        deps!();
        # [doc = " A literal HSTRING, length-prefixed wide string with a trailing null terminator."] # [macro_export] macro_rules ! h { ($ s : literal) => { { const INPUT : & [u8] = $ s . as_bytes () ; const OUTPUT_LEN : usize = $ crate :: utf16_len (INPUT) + 1 ; static RESULT : $ crate :: HSTRING = { if OUTPUT_LEN == 1 { unsafe { :: core :: mem :: transmute (:: core :: ptr :: null ::< u16 > ()) } } else { # [repr (C)] struct HSTRING_HEADER { flags : u32 , len : u32 , padding1 : u32 , padding2 : u32 , ptr : * const u16 , padding3 : i32 , padding4 : u16 , } const OUTPUT : $ crate :: PCWSTR = $ crate :: w ! ($ s) ; const HEADER : HSTRING_HEADER = HSTRING_HEADER { flags : 0x11 , len : (OUTPUT_LEN - 1) as u32 , padding1 : 0 , padding2 : 0 , ptr : OUTPUT . as_ptr () , padding3 : 0 , padding4 : 0 , } ; unsafe { :: core :: mem :: transmute ::<& HSTRING_HEADER , $ crate :: HSTRING > (& HEADER) } } } ; & RESULT } } ; }
    };
}

h!()
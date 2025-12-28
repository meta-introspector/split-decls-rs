macro_rules! deps {
    () => {
        Token!();
        Raw!();
        SourceIndex!();
        Source!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl SourceIndex for & crate :: lexer :: Token { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { let encoding = self . kind () . encoding () ; source . get_raw_str (self . span ()) . map (| s | Raw :: new_unchecked (s , encoding , self . span ())) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { let encoding = self . kind () . encoding () ; let raw = unsafe { source . get_raw_str_unchecked (self . span ()) } ; Raw :: new_unchecked (raw , encoding , self . span ()) } }
    };
}

impl_38!();
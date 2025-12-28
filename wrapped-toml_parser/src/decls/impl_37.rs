macro_rules! deps {
    () => {
        Token!();
        Raw!();
        Source!();
        SourceIndex!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl SourceIndex for crate :: lexer :: Token { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { (& self) . get (source) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { unsafe { (& self) . get_unchecked (source) } } }
    };
}

impl_37!()
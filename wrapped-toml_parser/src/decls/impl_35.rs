macro_rules! deps {
    () => {
        SourceIndex!();
        Span!();
        Raw!();
        Source!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl SourceIndex for Span { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { (& self) . get (source) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { unsafe { (& self) . get_unchecked (source) } } }
    };
}

impl_35!();
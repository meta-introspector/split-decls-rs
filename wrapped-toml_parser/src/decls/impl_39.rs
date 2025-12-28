macro_rules! deps {
    () => {
        Source!();
        SourceIndex!();
        Raw!();
        Event!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl SourceIndex for crate :: parser :: Event { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { (& self) . get (source) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { unsafe { (& self) . get_unchecked (source) } } }
    };
}

impl_39!()
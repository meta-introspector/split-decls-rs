macro_rules! deps {
    () => {
        Computed!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < S : ? Sized > Computed < S > { # [inline] fn new_borrowed < 'a > (stream : & 'a mut S) -> & 'a mut Computed < S > { unsafe { & mut * (stream as * mut _ as * mut Computed < S >) } } }
    };
}

impl_57!()
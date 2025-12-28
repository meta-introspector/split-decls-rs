macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < S > Deref for Ascii < S > { type Target = S ; # [inline] fn deref < 'a > (& 'a self) -> & 'a S { & self . 0 } }
    };
}

impl_4!();
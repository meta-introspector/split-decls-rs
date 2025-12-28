macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < S > DerefMut for Ascii < S > { # [inline] fn deref_mut < 'a > (& 'a mut self) -> & 'a mut S { & mut self . 0 } }
    };
}

impl_5!();
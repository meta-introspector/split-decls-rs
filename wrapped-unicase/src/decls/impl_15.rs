macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < S > DerefMut for UniCase < S > { # [inline] fn deref_mut < 'a > (& 'a mut self) -> & 'a mut S { inner ! (mut self . 0) } }
    };
}

impl_15!()
macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < S > Deref for UniCase < S > { type Target = S ; # [inline] fn deref < 'a > (& 'a self) -> & 'a S { inner ! (self . 0) } }
    };
}

impl_14!()
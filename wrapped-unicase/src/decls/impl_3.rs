macro_rules! deps {
    () => {
        UniCase!();
        Encoding!();
        Ascii!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < S > Ascii < S > { # [doc = " Construct a new `Ascii`."] # [doc = ""] # [doc = " For Rust versions >= 1.31, this is a `const fn`."] # [inline] pub const fn new (s : S) -> Ascii < S > { Ascii (s) } # [doc = " Convert this into a [`UniCase`]."] pub const fn into_unicase (self) -> UniCase < S > { UniCase (Encoding :: Ascii (self)) } # [doc = " Consume this `Ascii` and get the inner value."] # [inline] pub fn into_inner (self) -> S { self . 0 } }
    };
}

impl_3!()
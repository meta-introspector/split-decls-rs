macro_rules! deps {
    () => {
        Unicode!();
        UniCase!();
        Encoding!();
        Ascii!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < S > UniCase < S > { # [doc = " Creates a new `UniCase`, skipping the ASCII check."] pub const fn unicode (s : S) -> UniCase < S > { UniCase (Encoding :: Unicode (Unicode (s))) } # [doc = " Creates a new `UniCase` which performs only ASCII case folding."] pub const fn ascii (s : S) -> UniCase < S > { UniCase (Encoding :: Ascii (Ascii (s))) } # [doc = " Return `true` if this instance will only perform ASCII case folding."] pub fn is_ascii (& self) -> bool { match self . 0 { Encoding :: Ascii (_) => true , Encoding :: Unicode (_) => false , } } # [doc = " Unwraps the inner value held by this `UniCase`."] # [inline] pub fn into_inner (self) -> S { match self . 0 { Encoding :: Ascii (s) => s . 0 , Encoding :: Unicode (s) => s . 0 , } } }
    };
}

impl_40!()
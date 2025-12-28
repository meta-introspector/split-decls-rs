macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl FromStr for SmolStr { type Err = Infallible ; # [inline] fn from_str (s : & str) -> Result < SmolStr , Self :: Err > { Ok (SmolStr :: from (s)) } }
    };
}

impl_42!();
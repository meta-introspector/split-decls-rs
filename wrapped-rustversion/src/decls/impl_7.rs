macro_rules! deps {
    () => {
        Version!();
        Bound!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl PartialEq < Bound > for Version { fn eq (& self , rhs : & Bound) -> bool { match rhs { Bound :: Nightly (date) => match self . channel { Stable | Beta | Dev => false , Nightly (nightly) => nightly == * date , } , Bound :: Stable (release) => { self . minor == release . minor && release . patch . map_or (true , | patch | self . patch == patch) } } } }
    };
}

impl_7!()
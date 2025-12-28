macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < S : FromStr > FromStr for Ascii < S > { type Err = < S as FromStr > :: Err ; fn from_str (s : & str) -> Result < Ascii < S > , < S as FromStr > :: Err > { s . parse () . map (Ascii) } }
    };
}

impl_14!()
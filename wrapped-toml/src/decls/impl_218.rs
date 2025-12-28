macro_rules! deps {
    () => {
        DeFloat!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl DeFloat < '_ > { pub (crate) fn to_f64 (& self) -> Option < f64 > { let f : f64 = self . inner . as_ref () . parse () . ok () ? ; if f . is_infinite () && ! self . as_str () . contains ("inf") { None } else { Some (f) } } # [doc = " [`FromStr`][std::str::FromStr]-compatible representation of a float"] pub fn as_str (& self) -> & str { self . inner . as_ref () } }
    };
}

impl_218!();
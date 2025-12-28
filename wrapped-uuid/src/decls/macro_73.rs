macro_rules! deps {
    () => {
        Simple!();
        Braced!();
        Hyphenated!();
        Urn!();
    };
}

macro_rules! macro_73 {
    () => {
        deps!();
        impl_fmt_traits ! { Hyphenated <>, Simple <>, Urn <>, Braced <> }
    };
}

macro_73!();
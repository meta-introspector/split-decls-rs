macro_rules! deps {
    () => {
        Hyphenated!();
        Simple!();
        Urn!();
        Uuid!();
        Braced!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Uuid { # [doc = " Get a [`Hyphenated`] formatter."] # [inline] pub const fn hyphenated (self) -> Hyphenated { Hyphenated (self) } # [doc = " Get a borrowed [`Hyphenated`] formatter."] # [inline] pub fn as_hyphenated (& self) -> & Hyphenated { unsafe_transmute_ref ! (self) } # [doc = " Get a [`Simple`] formatter."] # [inline] pub const fn simple (self) -> Simple { Simple (self) } # [doc = " Get a borrowed [`Simple`] formatter."] # [inline] pub fn as_simple (& self) -> & Simple { unsafe_transmute_ref ! (self) } # [doc = " Get a [`Urn`] formatter."] # [inline] pub const fn urn (self) -> Urn { Urn (self) } # [doc = " Get a borrowed [`Urn`] formatter."] # [inline] pub fn as_urn (& self) -> & Urn { unsafe_transmute_ref ! (self) } # [doc = " Get a [`Braced`] formatter."] # [inline] pub const fn braced (self) -> Braced { Braced (self) } # [doc = " Get a borrowed [`Braced`] formatter."] # [inline] pub fn as_braced (& self) -> & Braced { unsafe_transmute_ref ! (self) } }
    };
}

impl_54!()
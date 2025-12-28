macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl FromStableHash for Fingerprint { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , _1]) : Self :: Hash) -> Self { Fingerprint (_0 , _1) } }
    };
}

impl_37!()
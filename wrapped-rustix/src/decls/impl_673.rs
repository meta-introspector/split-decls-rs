macro_rules! deps {
    () => {
        InlinedName!();
    };
}

macro_rules! impl_673 {
    () => {
        deps!();
        impl AsRef < str > for InlinedName { fn as_ref (& self) -> & str { core :: str :: from_utf8 (& self . name [.. self . len]) . unwrap () } }
    };
}

impl_673!();
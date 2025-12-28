macro_rules! deps {
    () => {
        Prerelease!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Deref for Prerelease { type Target = str ; fn deref (& self) -> & Self :: Target { self . identifier . as_str () } }
    };
}

impl_52!()
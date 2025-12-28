macro_rules! deps {
    () => {
        BuildMetadata!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Deref for BuildMetadata { type Target = str ; fn deref (& self) -> & Self :: Target { self . identifier . as_str () } }
    };
}

impl_53!();
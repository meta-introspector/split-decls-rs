macro_rules! deps {
    () => {
        PrefixOf!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'a > PrefixOf < 'a > { # [doc = " Creates a new `PrefixOf` from the given slice."] fn new (prefix_of : & 'a [u8]) -> Self { Self { prefix_of } } }
    };
}

impl_12!();
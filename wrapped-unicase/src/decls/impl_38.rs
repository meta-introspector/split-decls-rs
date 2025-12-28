macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < S : AsRef < str > + Default > Default for UniCase < S > { fn default () -> Self { Self :: new (Default :: default ()) } }
    };
}

impl_38!();
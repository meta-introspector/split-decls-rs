macro_rules! deps {
    () => {
        Tag!();
        EcParameters!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl FixedTag for EcParameters { const TAG : Tag = Tag :: ObjectIdentifier ; }
    };
}

impl_44!();
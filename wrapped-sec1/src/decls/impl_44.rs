macro_rules! deps {
    () => {
        EcParameters!();
        Tag!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl FixedTag for EcParameters { const TAG : Tag = Tag :: ObjectIdentifier ; }
    };
}

impl_44!()
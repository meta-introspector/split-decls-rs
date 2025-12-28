macro_rules! deps {
    () => {
        AstIdMap!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Eq for AstIdMap { }
    };
}

impl_47!()
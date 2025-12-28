macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl AstIdNode for ast :: ExternBlock { }
    };
}

impl_26!();
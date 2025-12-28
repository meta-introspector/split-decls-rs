macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl AstIdNode for ast :: Impl { }
    };
}

impl_32!()
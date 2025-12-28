macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl AstIdNode for ast :: Use { }
    };
}

impl_28!()
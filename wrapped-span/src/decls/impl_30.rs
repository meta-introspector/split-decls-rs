macro_rules! deps {
    () => {
        AstIdNode!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl AstIdNode for ast :: AsmExpr { }
    };
}

impl_30!();
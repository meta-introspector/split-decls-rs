macro_rules! deps {
    () => {
        BridgeTys!();
        Prov!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl rustc_public_bridge :: bridge :: Prov < compiler_interface :: BridgeTys > for crate :: ty :: Prov { fn new (aid : crate :: mir :: alloc :: AllocId) -> Self { Self (aid) } }
    };
}

impl_524!();
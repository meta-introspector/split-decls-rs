macro_rules! impl_58 {
    () => {
        impl rustc_public_bridge :: bridge :: Prov < compiler_interface :: BridgeTys > for crate :: ty :: Prov { fn new (aid : crate :: mir :: alloc :: AllocId) -> Self { Self (aid) } }
    };
}

impl_58!()
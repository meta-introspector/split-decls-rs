macro_rules! deps {
    () => {
        Transparency!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Transparency { pub fn fallback (macro_rules : bool) -> Self { if macro_rules { Transparency :: SemiOpaque } else { Transparency :: Opaque } } }
    };
}

impl_52!()
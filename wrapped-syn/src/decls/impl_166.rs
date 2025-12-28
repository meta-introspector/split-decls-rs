macro_rules! deps {
    () => {
        TrivialDrop!();
        IterMut!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < T > TrivialDrop for slice :: IterMut < '_ , T > { }
    };
}

impl_166!();
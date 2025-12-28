macro_rules! deps {
    () => {
        IterMut!();
        IntoIter!();
        TrivialDrop!();
        PrivateIterMut!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        impl < 'a , T , P > TrivialDrop for PrivateIterMut < 'a , T , P > where slice :: IterMut < 'a , (T , P) > : TrivialDrop , option :: IntoIter < & 'a mut T > : TrivialDrop , { }
    };
}

impl_652!();
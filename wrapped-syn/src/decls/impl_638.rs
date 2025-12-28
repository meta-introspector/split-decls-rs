macro_rules! deps {
    () => {
        TrivialDrop!();
        IntoIter!();
        Iter!();
        PrivateIter!();
    };
}

macro_rules! impl_638 {
    () => {
        deps!();
        impl < 'a , T , P > TrivialDrop for PrivateIter < 'a , T , P > where slice :: Iter < 'a , (T , P) > : TrivialDrop , option :: IntoIter < & 'a T > : TrivialDrop , { }
    };
}

impl_638!();
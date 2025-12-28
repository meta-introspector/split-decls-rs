macro_rules! deps {
    () => {
        UnordItems!();
    };
}

macro_rules! impl_639 {
    () => {
        deps!();
        impl < T > UnordItems < T , std :: iter :: Empty < T > > { pub fn empty () -> Self { UnordItems (std :: iter :: empty ()) } }
    };
}

impl_639!()
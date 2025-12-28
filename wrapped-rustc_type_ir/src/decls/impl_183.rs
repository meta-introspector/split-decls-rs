macro_rules! deps {
    () => {
        QueryInput!();
        Interner!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < I : Interner , P : Eq > Eq for QueryInput < I , P > { }
    };
}

impl_183!()
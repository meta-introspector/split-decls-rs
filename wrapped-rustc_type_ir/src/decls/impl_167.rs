macro_rules! deps {
    () => {
        State!();
        Interner!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < I : Interner , T : Eq > Eq for State < I , T > { }
    };
}

impl_167!()
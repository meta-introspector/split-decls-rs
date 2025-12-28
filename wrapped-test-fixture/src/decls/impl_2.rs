macro_rules! deps {
    () => {
        WithFixture!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < DB : ExpandDatabase + SourceDatabase + Default + 'static > WithFixture for DB { }
    };
}

impl_2!()
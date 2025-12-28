macro_rules! deps {
    () => {
        UnordItems!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl < T , I > ! IntoIterator for UnordItems < T , I > { }
    };
}

impl_675!()
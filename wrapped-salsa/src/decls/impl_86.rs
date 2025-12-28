macro_rules! deps {
    () => {
        DatabaseImpl!();
        Database!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Database for DatabaseImpl { }
    };
}

impl_86!()
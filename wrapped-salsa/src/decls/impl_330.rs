macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl Default for Table { fn default () -> Self { Self { pages : boxcar :: Vec :: new () , non_full_pages : Default :: default () , } } }
    };
}

impl_330!()
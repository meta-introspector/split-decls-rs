macro_rules! deps {
    () => {
        Adler32!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Default for Adler32 { fn default () -> Self { Self { a : 1 , b : 0 , update : get_imp () , } } }
    };
}

impl_53!();
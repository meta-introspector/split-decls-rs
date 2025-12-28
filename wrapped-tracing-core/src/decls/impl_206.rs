macro_rules! deps {
    () => {
        Level!();
        LevelFilter!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl From < LevelFilter > for Option < Level > { # [inline] fn from (filter : LevelFilter) -> Self { filter . into_level () } }
    };
}

impl_206!();
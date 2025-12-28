macro_rules! deps {
    () => {
        Level!();
        LevelFilter!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl From < Option < Level > > for LevelFilter { # [inline] fn from (level : Option < Level >) -> Self { Self (level) } }
    };
}

impl_205!()
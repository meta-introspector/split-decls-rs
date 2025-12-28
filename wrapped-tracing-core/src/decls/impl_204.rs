macro_rules! deps {
    () => {
        Level!();
        LevelFilter!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl From < Level > for LevelFilter { # [inline] fn from (level : Level) -> Self { Self :: from_level (level) } }
    };
}

impl_204!();
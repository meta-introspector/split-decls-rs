macro_rules! deps {
    () => {
        Level!();
        LevelFilter!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl PartialEq < Level > for LevelFilter { # [inline (always)] fn eq (& self , other : & Level) -> bool { filter_as_usize (& self . 0) == other . 0 as usize } }
    };
}

impl_220!();
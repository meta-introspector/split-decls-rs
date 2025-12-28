macro_rules! deps {
    () => {
        LevelFilter!();
        Level!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl PartialEq < LevelFilter > for Level { # [inline (always)] fn eq (& self , other : & LevelFilter) -> bool { self . 0 as usize == filter_as_usize (& other . 0) } }
    };
}

impl_215!()
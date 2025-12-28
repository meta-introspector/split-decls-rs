macro_rules! deps {
    () => {
        LittleEndianReadExt!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < R : Read > LittleEndianReadExt for R { }
    };
}

impl_349!();
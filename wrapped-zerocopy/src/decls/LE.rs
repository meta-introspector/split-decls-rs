macro_rules! deps {
    () => {
        LittleEndian!();
    };
}

macro_rules! LE {
    () => {
        deps!();
        # [doc = " A type alias for [`LittleEndian`]."] pub type LE = LittleEndian ;
    };
}

LE!();
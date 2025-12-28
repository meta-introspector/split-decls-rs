macro_rules! Endian {
    () => {
        # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) enum Endian { Little , Big , }
    };
}

Endian!();
macro_rules! Endian {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Serialize)] pub enum Endian { Little , Big , }
    };
}

Endian!();
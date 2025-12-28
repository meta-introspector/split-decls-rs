macro_rules! deps {
    () => {
        EndianWrapper!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl FromStr for EndianWrapper { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { rustc_abi :: Endian :: from_str (s) . map (Self) } }
    };
}

impl_440!()
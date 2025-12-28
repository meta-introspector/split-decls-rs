macro_rules! deps {
    () => {
        Bom!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl FromStr for Bom { # [doc = " A `std::io::Error` instance returned by `std::fs::File::open`."] type Err = Error ; # [doc = " Parse the BOM type from the file located at `path`."] fn from_str (path : & str) -> Result < Self , Self :: Err > { let mut file = File :: open (path) ? ; Ok (Bom :: from (& mut file)) } }
    };
}

impl_11!()
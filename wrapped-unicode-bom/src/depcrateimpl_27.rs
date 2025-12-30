// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl FromStr for Bom { # [doc = " A `std::io::Error` instance returned by `std::fs::File::open`."] type Err = Error ; # [doc = " Parse the BOM type from the file located at `path`."] fn from_str (path : & str) -> Result < Self , Self :: Err > { let mut file = File :: open (path) ? ; Ok (Bom :: from (& mut file)) } }
};
}

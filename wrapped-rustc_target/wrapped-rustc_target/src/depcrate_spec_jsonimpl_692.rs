// Generated macro for impl_692 (impl)
macro_rules! Depcrate_spec_jsonimpl_692 {
() => {
// Module: crate::spec::json
// Provides: {"impl_692"}
// Dependencies: {}
impl FromStr for EndianWrapper { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { rustc_abi :: Endian :: from_str (s) . map (Self) } }
};
}

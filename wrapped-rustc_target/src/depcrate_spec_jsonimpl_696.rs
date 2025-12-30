// Generated macro for impl_696 (impl)
macro_rules! Depcrate_spec_jsonimpl_696 {
() => {
// Module: crate::spec::json
// Provides: {"impl_696"}
// Dependencies: {}
impl FromStr for ExternAbiWrapper { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { rustc_abi :: ExternAbi :: from_str (s) . map (Self) . map_err (| _ | format ! ("{s} is not a valid extern ABI")) } }
};
}

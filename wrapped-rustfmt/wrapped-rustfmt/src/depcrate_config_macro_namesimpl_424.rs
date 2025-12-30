// Generated macro for impl_424 (impl)
macro_rules! Depcrate_config_macro_namesimpl_424 {
() => {
// Module: crate::config::macro_names
// Provides: {"impl_424"}
// Dependencies: {}
impl str :: FromStr for MacroSelectors { type Err = MacroSelectorsError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let raw : Vec < & str > = json :: from_str (s) . map_err (MacroSelectorsError :: Json) ? ; Ok (Self (raw . into_iter () . map (| raw | { MacroSelector :: from_str (raw) . expect ("MacroSelector from_str is infallible") }) . collect () ,)) } }
};
}

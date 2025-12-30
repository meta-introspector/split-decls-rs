// Generated macro for impl_420 (impl)
macro_rules! Depcrate_config_macro_namesimpl_420 {
() => {
// Module: crate::config::macro_names
// Provides: {"impl_420"}
// Dependencies: {}
impl str :: FromStr for MacroSelector { type Err = std :: convert :: Infallible ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { "*" => MacroSelector :: All , name => MacroSelector :: Name (MacroName (name . to_owned ())) , }) } }
};
}

// Generated macro for impl_138 (impl)
macro_rules! Depcrate_intrinsicimpl_138 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_138"}
// Dependencies: {}
impl FromStr for StaticDefinition { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s . trim () { s if s . starts_with ("const ") => Ok (StaticDefinition :: Constant (s [6 ..] . trim () . parse () ?)) , s => Ok (StaticDefinition :: Generic (s . to_string ())) , } } }
};
}

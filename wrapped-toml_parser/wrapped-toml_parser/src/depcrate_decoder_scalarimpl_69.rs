// Generated macro for impl_69 (impl)
macro_rules! Depcrate_decoder_scalarimpl_69 {
() => {
// Module: crate::decoder::scalar
// Provides: {"impl_69"}
// Dependencies: {}
impl ScalarKind { pub fn description (& self) -> & 'static str { match self { Self :: String => "string" , Self :: Boolean (_) => "boolean" , Self :: DateTime => "date-time" , Self :: Float => "float" , Self :: Integer (radix) => radix . description () , } } pub fn invalid_description (& self) -> & 'static str { match self { Self :: String => "invalid string" , Self :: Boolean (_) => "invalid boolean" , Self :: DateTime => "invalid date-time" , Self :: Float => "invalid float" , Self :: Integer (radix) => radix . invalid_description () , } } }
};
}

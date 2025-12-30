// Generated macro for impl_79 (impl)
macro_rules! Depcrate_libs_serde_jsonimpl_79 {
() => {
// Module: crate::libs::serde_json
// Provides: {"impl_79"}
// Dependencies: {}
impl TypeSize for serde_json :: Value { fn extra_size (& self) -> usize { match self { Self :: Null => 0 , Self :: Bool (value) => value . extra_size () , Self :: Number (value) => value . extra_size () , Self :: String (value) => value . extra_size () , Self :: Array (value) => value . extra_size () , Self :: Object (value) => value . extra_size () , } } }
};
}

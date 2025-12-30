// Generated macro for impl_209 (impl)
macro_rules! Depcrate_filter_utilsimpl_209 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_209"}
// Dependencies: {}
impl < K : GetValue + Eq + std :: hash :: Hash > UniqueStrategy for Unique < K > { fn insert (& mut self , val : & Value) -> Result < bool > { Ok (self . unique . insert (K :: get_value (val) ?)) } }
};
}

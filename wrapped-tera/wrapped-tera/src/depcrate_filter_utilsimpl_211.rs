// Generated macro for impl_211 (impl)
macro_rules! Depcrate_filter_utilsimpl_211 {
() => {
// Module: crate::filter_utils
// Provides: {"impl_211"}
// Dependencies: {}
impl UniqueStrategy for UniqueStrings { fn insert (& mut self , val : & Value) -> Result < bool > { let mut key = String :: get_value (val) ? ; if ! self . case_sensitive { key = key . to_lowercase () } Ok (self . u . unique . insert (key)) } }
};
}

// Generated macro for impl_114 (impl)
macro_rules! Depcrate_data_sourceimpl_114 {
() => {
// Module: crate::data::source
// Provides: {"impl_114"}
// Dependencies: {}
impl Inline { pub (crate) fn trimmed (& self) -> String { let mut data = self . data ; if data . contains ('\n') { data = data . strip_prefix ('\n') . unwrap_or (data) ; data = data . strip_suffix ('\n') . unwrap_or (data) ; } data . to_owned () } }
};
}

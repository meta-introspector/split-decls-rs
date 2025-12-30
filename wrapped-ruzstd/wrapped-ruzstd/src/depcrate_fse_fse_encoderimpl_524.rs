// Generated macro for impl_524 (impl)
macro_rules! Depcrate_fse_fse_encoderimpl_524 {
() => {
// Module: crate::fse::fse_encoder
// Provides: {"impl_524"}
// Dependencies: {}
impl SymbolStates { fn get (& self , idx : usize , max_idx : usize) -> & State { let start_search_at = (idx * self . states . len ()) / max_idx ; self . states [start_search_at ..] . iter () . find (| state | state . contains (idx)) . unwrap () } }
};
}

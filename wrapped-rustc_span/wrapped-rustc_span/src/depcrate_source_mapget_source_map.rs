// Generated macro for get_source_map (function)
macro_rules! Depcrate_source_mapget_source_map {
() => {
// Module: crate::source_map
// Provides: {"get_source_map"}
// Dependencies: {}
pub fn get_source_map () -> Option < Arc < SourceMap > > { with_session_globals (| session_globals | session_globals . source_map . clone ()) }
};
}

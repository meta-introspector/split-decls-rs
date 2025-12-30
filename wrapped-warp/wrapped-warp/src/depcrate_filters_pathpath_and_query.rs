// Generated macro for path_and_query (function)
macro_rules! Depcrate_filters_pathpath_and_query {
() => {
// Module: crate::filters::path
// Provides: {"path_and_query"}
// Dependencies: {}
fn path_and_query (route : & Route) -> PathAndQuery { route . uri () . path_and_query () . cloned () . unwrap_or_else (| | PathAndQuery :: from_static ("")) }
};
}

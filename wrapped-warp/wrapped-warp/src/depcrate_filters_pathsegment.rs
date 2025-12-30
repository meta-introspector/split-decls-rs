// Generated macro for segment (function)
macro_rules! Depcrate_filters_pathsegment {
() => {
// Module: crate::filters::path
// Provides: {"segment"}
// Dependencies: {}
fn segment (route : & Route) -> & str { route . path () . splitn (2 , '/') . next () . expect ("split always has at least 1") }
};
}

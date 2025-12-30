// Generated macro for regex (macro)
macro_rules! Depcrate_framework_graphvizregex {
() => {
// Module: crate::framework::graphviz
// Provides: {"regex"}
// Dependencies: {}
macro_rules ! regex { ($ re : literal $ (,) ?) => { { static RE : OnceLock < regex :: Regex > = OnceLock :: new () ; RE . get_or_init (|| Regex :: new ($ re) . unwrap ()) } } ; }
};
}

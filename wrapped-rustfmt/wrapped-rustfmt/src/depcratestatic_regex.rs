// Generated macro for static_regex (macro)
macro_rules! Depcratestatic_regex {
() => {
// Module: crate
// Provides: {"static_regex"}
// Dependencies: {}
macro_rules ! static_regex { ($ re : literal) => { { static RE : :: std :: sync :: OnceLock <:: regex :: Regex > = :: std :: sync :: OnceLock :: new () ; RE . get_or_init (|| :: regex :: Regex :: new ($ re) . unwrap ()) } } ; }
};
}

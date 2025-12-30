// Generated macro for static_regex (macro)
macro_rules! Depcratestatic_regex {
() => {
// Module: crate
// Provides: {"static_regex"}
// Dependencies: {}
macro_rules ! static_regex { ($ re : literal) => { { static RE : :: std :: sync :: LazyLock <:: regex :: Regex > = :: std :: sync :: LazyLock :: new (|| :: regex :: Regex :: new ($ re) . unwrap ()) ; &* RE } } ; }
};
}

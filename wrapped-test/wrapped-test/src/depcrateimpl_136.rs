// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
impl Component { # [doc = " Helper to convert `RuntimeTestConfig` to a `RuntimeTestConfig<T>` and"] # [doc = " then extract the `T`."] # [doc = ""] # [doc = " This is called from within each language's implementation with a"] # [doc = " specific `T` necessary for that language."] fn deserialize_lang_config < T > (& self) -> Result < T > where T : Default + serde :: de :: DeserializeOwned , { if self . lang_config . is_none () { return Ok (T :: default ()) ; } let config = config :: parse_test_config :: < config :: RuntimeTestConfig < T > > (& self . contents , self . language . obj () . comment_prefix_for_test_config () . unwrap () ,) ? ; Ok (config . lang . unwrap ()) } }
};
}

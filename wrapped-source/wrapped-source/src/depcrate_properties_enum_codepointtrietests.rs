// Generated macro for tests (module)
macro_rules! Depcrate_properties_enum_codepointtrietests {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_general_category () { use icu :: properties :: { props :: GeneralCategory , CodePointMapData } ; let provider = SourceDataProvider :: new_testing () ; let trie = CodePointMapData :: < GeneralCategory > :: try_new_unstable (& provider) . unwrap () ; let trie = trie . as_code_point_trie () . unwrap () ; assert_eq ! (trie . get32 ('꣓' as u32) , GeneralCategory :: DecimalNumber) ; assert_eq ! (trie . get32 ('≈' as u32) , GeneralCategory :: MathSymbol) ; } # [test] fn test_script () { use icu :: properties :: { props :: Script , CodePointMapData } ; let provider = SourceDataProvider :: new_testing () ; let trie = CodePointMapData :: < Script > :: try_new_unstable (& provider) . unwrap () ; let trie = trie . as_code_point_trie () . unwrap () ; assert_eq ! (trie . get32 ('꣓' as u32) , Script :: Saurashtra) ; assert_eq ! (trie . get32 ('≈' as u32) , Script :: Common) ; } }
};
}

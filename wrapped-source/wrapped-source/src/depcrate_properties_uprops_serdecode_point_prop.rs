// Generated macro for code_point_prop (module)
macro_rules! Depcrate_properties_uprops_serdecode_point_prop {
() => {
// Module: crate::properties::uprops_serde
// Provides: {"code_point_prop"}
// Dependencies: {}
# [cfg (any (feature = "use_wasm" , feature = "use_icu4c"))] pub (crate) mod code_point_prop { # [derive (serde :: Deserialize)] pub (crate) struct CodePointPropertyMap { pub (crate) code_point_trie : super :: CodePointTrieToml , } # [derive (serde :: Deserialize)] pub (crate) struct Main { # [serde (default)] pub (crate) enum_property : Vec < CodePointPropertyMap > , } }
};
}

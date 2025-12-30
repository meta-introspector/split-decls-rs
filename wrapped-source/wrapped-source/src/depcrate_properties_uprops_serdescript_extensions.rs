// Generated macro for script_extensions (module)
macro_rules! Depcrate_properties_uprops_serdescript_extensions {
() => {
// Module: crate::properties::uprops_serde
// Provides: {"script_extensions"}
// Dependencies: {}
pub (crate) mod script_extensions { use super :: CodePointTrieToml ; # [derive (serde :: Deserialize)] pub (crate) struct ScriptWithExtensionsPropertyProperty { # [serde (rename = "long_name")] pub (crate) _long_name : String , # [serde (rename = "short_name")] pub (crate) _short_name : String , pub (crate) script_code_array : Vec < Vec < u16 > > , pub (crate) code_point_trie : CodePointTrieToml , } # [derive (serde :: Deserialize)] pub (crate) struct Main { # [serde (default)] pub (crate) script_extensions : Vec < ScriptWithExtensionsPropertyProperty > , } }
};
}

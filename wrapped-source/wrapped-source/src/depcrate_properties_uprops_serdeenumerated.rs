// Generated macro for enumerated (module)
macro_rules! Depcrate_properties_uprops_serdeenumerated {
() => {
// Module: crate::properties::uprops_serde
// Provides: {"enumerated"}
// Dependencies: {}
pub (crate) mod enumerated { # [derive (serde :: Deserialize)] pub (crate) struct EnumeratedPropertyMapRange { # [serde (rename = "a")] pub (crate) _a : u32 , # [serde (rename = "b")] pub (crate) _b : u32 , # [serde (rename = "v")] pub (crate) _v : u32 , # [serde (rename = "name")] pub (crate) _name : String , } # [derive (serde :: Deserialize)] pub (crate) struct EnumeratedPropertyMap { # [serde (rename = "long_name")] pub (crate) _long_name : String , # [serde (rename = "short_name")] pub (crate) _short_name : String , pub (crate) values : Vec < super :: PropertyValue > , # [serde (rename = "ranges")] pub (crate) _ranges : Vec < EnumeratedPropertyMapRange > , pub (crate) code_point_trie : super :: CodePointTrieToml , } # [derive (serde :: Deserialize)] pub (crate) struct Main { # [serde (default)] pub (crate) enum_property : Vec < EnumeratedPropertyMap > , } }
};
}

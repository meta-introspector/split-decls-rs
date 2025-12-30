// Generated macro for Dependency (struct)
macro_rules! Depcrate_dependenciesDependency {
() => {
// Module: crate::dependencies
// Provides: {"Dependency"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Debug)] # [serde (remote = "Self")] pub (crate) struct Dependency { # [serde (skip_serializing_if = "Option::is_none")] pub version : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub path : Option < Directory > , # [serde (default , skip_serializing_if = "is_false")] pub optional : bool , # [serde (rename = "default-features" , skip_serializing_if = "Option::is_none")] pub default_features : Option < bool > , # [serde (default , skip_serializing_if = "Vec::is_empty")] pub features : Vec < String > , # [serde (skip_serializing_if = "Option::is_none")] pub git : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub branch : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub tag : Option < String > , # [serde (skip_serializing_if = "Option::is_none")] pub rev : Option < String > , # [serde (default , skip_serializing_if = "is_false")] pub workspace : bool , # [serde (flatten)] pub rest : Map < String , Value > , }
};
}

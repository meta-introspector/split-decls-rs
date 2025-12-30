// Generated macro for Manifest (struct)
macro_rules! Depcrate_dependenciesManifest {
() => {
// Module: crate::dependencies
// Provides: {"Manifest"}
// Dependencies: {}
# [derive (Deserialize , Default , Debug)] pub (crate) struct Manifest { # [serde (rename = "cargo-features" , default)] pub cargo_features : Vec < String > , # [serde (default)] pub package : Package , # [serde (default)] pub features : Map < String , Vec < String > > , # [serde (default)] pub dependencies : Map < String , Dependency > , # [serde (default , alias = "dev-dependencies")] pub dev_dependencies : Map < String , Dependency > , # [serde (default)] pub target : Map < String , TargetDependencies > , }
};
}

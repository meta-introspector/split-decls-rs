// Generated macro for Package (struct)
macro_rules! Depcrate_dependenciesPackage {
() => {
// Module: crate::dependencies
// Provides: {"Package"}
// Dependencies: {}
# [derive (Deserialize , Default , Debug)] pub (crate) struct Package { pub name : String , # [serde (default)] pub edition : EditionOrInherit , pub resolver : Option < String > , }
};
}

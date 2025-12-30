// Generated macro for Env (struct)
macro_rules! Depcrate_schemaEnv {
() => {
// Module: crate::schema
// Provides: {"Env"}
// Dependencies: {}
# [doc = " Describe command's environment"] # [derive (Clone , Default , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] pub struct Env { # [serde (default)] pub (crate) inherit : Option < bool > , # [serde (default)] pub (crate) add : BTreeMap < String , String > , # [serde (default)] pub (crate) remove : Vec < String > , }
};
}

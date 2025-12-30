// Generated macro for Args (enum)
macro_rules! Depcrate_schemaArgs {
() => {
// Module: crate::schema
// Provides: {"Args"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] # [serde (untagged)] pub (crate) enum Args { Joined (JoinedArgs) , Split (Vec < String >) , }
};
}

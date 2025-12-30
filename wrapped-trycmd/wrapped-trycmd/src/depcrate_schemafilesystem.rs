// Generated macro for Filesystem (struct)
macro_rules! Depcrate_schemaFilesystem {
() => {
// Module: crate::schema
// Provides: {"Filesystem"}
// Dependencies: {}
# [doc = " Describe the command's filesystem context"] # [derive (Clone , Default , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] pub struct Filesystem { pub (crate) cwd : Option < std :: path :: PathBuf > , # [doc = " Sandbox base"] pub (crate) base : Option < std :: path :: PathBuf > , pub (crate) sandbox : Option < bool > , }
};
}

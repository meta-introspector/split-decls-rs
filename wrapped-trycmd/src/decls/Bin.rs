macro_rules! Bin {
    () => {
        # [doc = " Target under test"] # [derive (Clone , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] pub enum Bin { Path (std :: path :: PathBuf) , Name (String) , Ignore , # [serde (skip)] Error (crate :: Error) , }
    };
}

Bin!();
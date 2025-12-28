macro_rules! Env {
    () => {
        # [doc = " Describe command's environment"] # [derive (Clone , Default , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] pub struct Env { # [serde (default)] pub (crate) inherit : Option < bool > , # [serde (default)] pub (crate) add : BTreeMap < String , String > , # [serde (default)] pub (crate) remove : Vec < String > , }
    };
}

Env!()
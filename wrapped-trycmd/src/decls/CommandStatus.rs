macro_rules! CommandStatus {
    () => {
        # [doc = " Expected status for command"] # [derive (Copy , Clone , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] # [derive (Default)] pub enum CommandStatus { # [default] Success , Failed , Interrupted , Skipped , Code (i32) , }
    };
}

CommandStatus!();
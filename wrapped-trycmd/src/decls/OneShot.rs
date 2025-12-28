macro_rules! deps {
    () => {
        Args!();
        Bin!();
        Env!();
        CommandStatus!();
        Filesystem!();
    };
}

macro_rules! OneShot {
    () => {
        deps!();
        # [doc = " Top-level data in `cmd.toml` files"] # [derive (Clone , Default , Debug , PartialEq , Eq , serde :: Deserialize , serde :: Serialize)] # [serde (rename_all = "kebab-case")] # [cfg_attr (feature = "schema" , derive (schemars :: JsonSchema))] pub struct OneShot { pub (crate) bin : Option < Bin > , # [serde (default)] pub (crate) args : Args , # [serde (default)] pub (crate) env : Env , # [serde (default)] pub (crate) stdin : Option < String > , # [serde (default)] pub (crate) stdout : Option < String > , # [serde (default)] pub (crate) stderr : Option < String > , # [serde (default)] pub (crate) stderr_to_stdout : bool , pub (crate) status : Option < CommandStatus > , # [serde (default)] pub (crate) binary : bool , # [serde (default)] # [serde (deserialize_with = "humantime_serde::deserialize")] pub (crate) timeout : Option < std :: time :: Duration > , # [serde (default)] pub (crate) fs : Filesystem , }
    };
}

OneShot!();
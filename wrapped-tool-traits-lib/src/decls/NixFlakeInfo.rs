macro_rules! NixFlakeInfo {
    () => {
        # [derive (Debug , Default , PartialEq , Eq , Clone , Hash)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct NixFlakeInfo { pub flake_path : PathBuf , pub inputs : BTreeMap < String , String > , pub outputs : Vec < String > , }
    };
}

NixFlakeInfo!()
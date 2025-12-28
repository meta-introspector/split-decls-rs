macro_rules! SubmoduleInfo {
    () => {
        # [derive (Debug , Default , PartialEq , Eq , Clone , Hash)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct SubmoduleInfo { pub name : String , pub path : PathBuf , pub url : String , pub branch : Option < String > , pub commit_id : String , }
    };
}

SubmoduleInfo!()
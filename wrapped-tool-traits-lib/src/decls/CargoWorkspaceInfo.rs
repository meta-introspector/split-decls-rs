macro_rules! deps {
    () => {
        PackageInfo!();
    };
}

macro_rules! CargoWorkspaceInfo {
    () => {
        deps!();
        # [derive (Debug , Default , PartialEq , Eq , Clone , Hash)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct CargoWorkspaceInfo { pub manifest_path : PathBuf , pub packages : Vec < PackageInfo > , }
    };
}

CargoWorkspaceInfo!()
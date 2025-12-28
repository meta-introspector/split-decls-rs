macro_rules! deps {
    () => {
        DependencyInfo!();
    };
}

macro_rules! PackageInfo {
    () => {
        deps!();
        # [derive (Debug , Default , PartialEq , Eq , Clone , Hash)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct PackageInfo { pub name : String , pub version : String , pub manifest_path : PathBuf , pub dependencies : Vec < DependencyInfo > , }
    };
}

PackageInfo!();
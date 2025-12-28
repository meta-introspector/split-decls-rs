macro_rules! DependencyInfo {
    () => {
        # [derive (Debug , Default , PartialEq , Eq , Clone , Hash)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct DependencyInfo { pub name : String , pub source : String , pub req : String , }
    };
}

DependencyInfo!()
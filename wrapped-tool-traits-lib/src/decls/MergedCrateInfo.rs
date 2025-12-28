macro_rules! MergedCrateInfo {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct MergedCrateInfo { pub layer : i32 , pub usage_count : u32 , }
    };
}

MergedCrateInfo!()
macro_rules! IterationCount {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , Default , PartialOrd , Ord)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "persistence" , serde (transparent))] pub struct IterationCount (u8) ;
    };
}

IterationCount!();
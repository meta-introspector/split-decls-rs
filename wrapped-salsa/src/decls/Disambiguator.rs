macro_rules! Disambiguator {
    () => {
        # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Copy , Clone)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "persistence" , serde (transparent))] pub struct Disambiguator (u32) ;
    };
}

Disambiguator!();
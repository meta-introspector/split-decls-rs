macro_rules! salsa {
    () => {
        # [cfg (not (feature = "salsa"))] mod salsa { # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct Id (u32) ; }
    };
}

salsa!();
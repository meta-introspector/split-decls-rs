macro_rules! deps {
    () => {
        Ord!();
        Eq!();
    };
}

macro_rules! B0 {
    () => {
        deps!();
        # [doc = " The type-level bit 0."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct B0 ;
    };
}

B0!();
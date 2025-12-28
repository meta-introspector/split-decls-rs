macro_rules! deps {
    () => {
        Ord!();
        Eq!();
    };
}

macro_rules! Z0 {
    () => {
        deps!();
        # [doc = " The type-level signed integer 0."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct Z0 ;
    };
}

Z0!();
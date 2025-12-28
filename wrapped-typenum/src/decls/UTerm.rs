macro_rules! deps {
    () => {
        Ord!();
        UInt!();
        Eq!();
    };
}

macro_rules! UTerm {
    () => {
        deps!();
        # [doc = " The terminating type for `UInt`; it always comes after the most significant"] # [doc = " bit. `UTerm` by itself represents zero, which is aliased to `U0`."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct UTerm ;
    };
}

UTerm!()
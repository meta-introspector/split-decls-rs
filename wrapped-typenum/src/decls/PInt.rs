macro_rules! deps {
    () => {
        NonZero!();
        Eq!();
        Unsigned!();
        Ord!();
    };
}

macro_rules! PInt {
    () => {
        deps!();
        # [doc = " Type-level signed integers with positive sign."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct PInt < U : Unsigned + NonZero > { pub (crate) n : U , }
    };
}

PInt!()
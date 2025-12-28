macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        Ord!();
        Eq!();
    };
}

macro_rules! NInt {
    () => {
        deps!();
        # [doc = " Type-level signed integers with negative sign."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct NInt < U : Unsigned + NonZero > { pub (crate) n : U , }
    };
}

NInt!()
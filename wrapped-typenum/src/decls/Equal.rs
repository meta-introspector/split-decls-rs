macro_rules! deps {
    () => {
        Cmp!();
        Eq!();
        Ord!();
    };
}

macro_rules! Equal {
    () => {
        deps!();
        # [doc = " A potential output from `Cmp`, this is the type equivalent to the enum variant"] # [doc = " `core::cmp::Ordering::Equal`."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct Equal ;
    };
}

Equal!();
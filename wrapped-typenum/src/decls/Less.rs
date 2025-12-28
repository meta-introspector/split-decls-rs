macro_rules! deps {
    () => {
        Cmp!();
        Ord!();
        Eq!();
    };
}

macro_rules! Less {
    () => {
        deps!();
        # [doc = " A potential output from `Cmp`, this is the type equivalent to the enum variant"] # [doc = " `core::cmp::Ordering::Less`."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug , Default)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct Less ;
    };
}

Less!()
macro_rules! deps {
    () => {
        Ord!();
        Eq!();
    };
}

macro_rules! ATerm {
    () => {
        deps!();
        # [doc = " The terminating type for type arrays."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct ATerm ;
    };
}

ATerm!()
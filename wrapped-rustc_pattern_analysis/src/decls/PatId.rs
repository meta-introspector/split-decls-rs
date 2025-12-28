macro_rules! PatId {
    () => {
        # [doc = " A globally unique id to distinguish patterns."] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub (crate) struct PatId (u32) ;
    };
}

PatId!();
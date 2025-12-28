macro_rules! DefId {
    () => {
        # [doc = " A unique identification number for each item accessible for the current compilation unit."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct DefId (pub (crate) usize) ;
    };
}

DefId!();
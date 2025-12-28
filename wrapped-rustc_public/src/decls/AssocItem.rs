macro_rules! deps {
    () => {
        AssocContainer!();
        AssocKind!();
    };
}

macro_rules! AssocItem {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AssocItem { pub def_id : AssocDef , pub kind : AssocKind , pub container : AssocContainer , }
    };
}

AssocItem!()
macro_rules! DynKind {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum DynKind { Dyn , }
    };
}

DynKind!()
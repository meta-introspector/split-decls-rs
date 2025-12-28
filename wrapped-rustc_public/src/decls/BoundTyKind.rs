macro_rules! BoundTyKind {
    () => {
        # [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub enum BoundTyKind { Anon , Param (ParamDef , String) , }
    };
}

BoundTyKind!();
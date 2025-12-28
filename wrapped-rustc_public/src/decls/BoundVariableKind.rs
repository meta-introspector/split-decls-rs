macro_rules! deps {
    () => {
        Ty!();
        BoundTyKind!();
        Region!();
        BoundRegionKind!();
    };
}

macro_rules! BoundVariableKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum BoundVariableKind { Ty (BoundTyKind) , Region (BoundRegionKind) , Const , }
    };
}

BoundVariableKind!()
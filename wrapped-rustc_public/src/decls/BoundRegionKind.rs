macro_rules! BoundRegionKind {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum BoundRegionKind { BrAnon , BrNamed (BrNamedDef , String) , BrEnv , }
    };
}

BoundRegionKind!();
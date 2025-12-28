macro_rules! deps {
    () => {
        Opaque!();
    };
}

macro_rules! AliasKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AliasKind { Projection , Inherent , Opaque , Free , }
    };
}

AliasKind!();
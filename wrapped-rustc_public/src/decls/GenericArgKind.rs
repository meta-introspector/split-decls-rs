macro_rules! deps {
    () => {
        TyConst!();
        Region!();
        Ty!();
    };
}

macro_rules! GenericArgKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum GenericArgKind { Lifetime (Region) , Type (Ty) , Const (TyConst) , }
    };
}

GenericArgKind!()
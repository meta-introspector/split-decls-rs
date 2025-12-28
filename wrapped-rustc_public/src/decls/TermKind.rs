macro_rules! deps {
    () => {
        Ty!();
        TyConst!();
    };
}

macro_rules! TermKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TermKind { Type (Ty) , Const (TyConst) , }
    };
}

TermKind!();
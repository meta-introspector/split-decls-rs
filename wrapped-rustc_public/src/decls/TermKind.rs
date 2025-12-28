macro_rules! deps {
    () => {
        TyConst!();
        Ty!();
    };
}

macro_rules! TermKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TermKind { Type (Ty) , Const (TyConst) , }
    };
}

TermKind!()
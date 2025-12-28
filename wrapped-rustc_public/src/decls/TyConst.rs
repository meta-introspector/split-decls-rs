macro_rules! deps {
    () => {
        TyConstKind!();
        TyConstId!();
    };
}

macro_rules! TyConst {
    () => {
        deps!();
        # [doc = " Represents a constant in the type system"] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct TyConst { pub (crate) kind : TyConstKind , pub id : TyConstId , }
    };
}

TyConst!()
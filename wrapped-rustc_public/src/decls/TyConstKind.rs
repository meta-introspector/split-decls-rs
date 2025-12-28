macro_rules! deps {
    () => {
        DebruijnIndex!();
        Allocation!();
        BoundVar!();
        Ty!();
        ParamConst!();
        GenericArgs!();
    };
}

macro_rules! TyConstKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum TyConstKind { Param (ParamConst) , Bound (DebruijnIndex , BoundVar) , Unevaluated (ConstDef , GenericArgs) , Value (Ty , Allocation) , ZSTValue (Ty) , }
    };
}

TyConstKind!();
macro_rules! deps {
    () => {
        AliasKind!();
        AliasTy!();
        ParamTy!();
        BoundTy!();
        RigidTy!();
    };
}

macro_rules! TyKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TyKind { RigidTy (RigidTy) , Alias (AliasKind , AliasTy) , Param (ParamTy) , Bound (usize , BoundTy) , }
    };
}

TyKind!();
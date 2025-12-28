macro_rules! deps {
    () => {
        Interner!();
        Ty!();
    };
}

macro_rules! ValidateBoundVars {
    () => {
        deps!();
        pub struct ValidateBoundVars < I : Interner > { bound_vars : I :: BoundVarKinds , binder_index : ty :: DebruijnIndex , visited : SsoHashSet < (ty :: DebruijnIndex , I :: Ty) > , }
    };
}

ValidateBoundVars!()
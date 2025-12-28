macro_rules! deps {
    () => {
        ValidateBoundVars!();
        Interner!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < I : Interner > ValidateBoundVars < I > { pub fn new (bound_vars : I :: BoundVarKinds) -> Self { ValidateBoundVars { bound_vars , binder_index : ty :: INNERMOST , visited : SsoHashSet :: default () , } } }
    };
}

impl_230!();
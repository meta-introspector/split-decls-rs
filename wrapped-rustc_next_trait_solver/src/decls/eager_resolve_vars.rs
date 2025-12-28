macro_rules! deps {
    () => {
        EagerResolver!();
        SolverDelegate!();
    };
}

macro_rules! eager_resolve_vars {
    () => {
        deps!();
        pub fn eager_resolve_vars < D : SolverDelegate , T : TypeFoldable < D :: Interner > > (delegate : & D , value : T ,) -> T { if value . has_infer () { let mut folder = EagerResolver :: new (delegate) ; value . fold_with (& mut folder) } else { value } }
    };
}

eager_resolve_vars!()
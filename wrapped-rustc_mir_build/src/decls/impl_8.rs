macro_rules! deps {
    () => {
        ForGuard!();
        Builder!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , 'tcx > Builder < 'a , 'tcx > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { self . infcx . typing_env (self . param_env) } fn is_bound_var_in_guard (& self , id : LocalVarId) -> bool { self . guard_context . iter () . any (| frame | frame . locals . iter () . any (| local | local . id == id)) } fn var_local_id (& self , id : LocalVarId , for_guard : ForGuard) -> Local { self . var_indices [& id] . local_id (for_guard) } }
    };
}

impl_8!();
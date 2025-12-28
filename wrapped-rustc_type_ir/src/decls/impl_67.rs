macro_rules! deps {
    () => {
        ConstKind!();
        Ty!();
        ParamEnv!();
        ClauseKind!();
        Interner!();
        PlaceholderConst!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < I : Interner > PlaceholderConst < I > for I :: PlaceholderConst { fn find_const_ty_from_env (self , env : I :: ParamEnv) -> I :: Ty { let mut candidates = env . caller_bounds () . iter () . filter_map (| clause | { match clause . kind () . skip_binder () { ty :: ClauseKind :: ConstArgHasType (placeholder_ct , ty) => { assert ! (! (placeholder_ct , ty) . has_escaping_bound_vars ()) ; match placeholder_ct . kind () { ty :: ConstKind :: Placeholder (placeholder_ct) if placeholder_ct == self => { Some (ty) } _ => None , } } _ => None , } }) ; let ty = candidates . next () . unwrap_or_else (| | { panic ! ("cannot find `{self:?}` in param-env: {env:#?}") ; }) ; assert ! (candidates . next () . is_none () , "did not expect duplicate `ConstParamHasTy` for `{self:?}` in param-env: {env:#?}") ; ty } }
    };
}

impl_67!();
macro_rules! deps {
    () => {
        TypeVisitor!();
        Component!();
        OutlivesCollector!();
        Region!();
        Interner!();
        GenericArgKind!();
        Ty!();
        Const!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < I : Interner > TypeVisitor < I > for OutlivesCollector < '_ , I > { # [cfg (not (feature = "nightly"))] type Result = () ; fn visit_ty (& mut self , ty : I :: Ty) -> Self :: Result { if ! self . visited . insert (ty) { return ; } match ty . kind () { ty :: FnDef (_ , args) => { for child in args . iter () { match child . kind () { ty :: GenericArgKind :: Lifetime (_) => { } ty :: GenericArgKind :: Type (_) | ty :: GenericArgKind :: Const (_) => { child . visit_with (self) ; } } } } ty :: Closure (_ , args) => { args . as_closure () . tupled_upvars_ty () . visit_with (self) ; } ty :: CoroutineClosure (_ , args) => { args . as_coroutine_closure () . tupled_upvars_ty () . visit_with (self) ; } ty :: Coroutine (_ , args) => { args . as_coroutine () . tupled_upvars_ty () . visit_with (self) ; args . as_coroutine () . resume_ty () . visit_with (self) ; } ty :: CoroutineWitness (..) => { } ty :: Param (p) => { self . out . push (Component :: Param (p)) ; } ty :: Placeholder (p) => { self . out . push (Component :: Placeholder (p)) ; } ty :: Alias (kind , alias_ty) => { if ! alias_ty . has_escaping_bound_vars () { self . out . push (Component :: Alias (alias_ty)) ; } else { let mut subcomponents = smallvec ! [] ; compute_alias_components_recursive (self . cx , kind , alias_ty , & mut subcomponents) ; self . out . push (Component :: EscapingAlias (subcomponents . into_iter () . collect ())) ; } } ty :: Infer (infer_ty) => { self . out . push (Component :: UnresolvedInferenceVariable (infer_ty)) ; } ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str | ty :: Never | ty :: Error (_) => { } ty :: Bound (_ , _) => { } ty :: Adt (_ , _) | ty :: Foreign (_) | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: Ref (_ , _ , _) | ty :: FnPtr (..) | ty :: UnsafeBinder (_) | ty :: Dynamic (_ , _ , _) | ty :: Tuple (_) => { ty . super_visit_with (self) ; } } } fn visit_region (& mut self , lt : I :: Region) -> Self :: Result { if ! lt . is_bound () { self . out . push (Component :: Region (lt)) ; } } }
    };
}

impl_103!()
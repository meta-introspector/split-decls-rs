macro_rules! deps {
    () => {
        SolverDelegate!();
        FindParamInClause!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < D , I > TypeVisitor < I > for FindParamInClause < '_ , '_ , D , I > where D : SolverDelegate < Interner = I > , I : Interner , { type Result = ControlFlow < Result < () , NoSolution > > ; fn visit_binder < T : TypeVisitable < I > > (& mut self , t : & ty :: Binder < I , T >) -> Self :: Result { self . universes . push (None) ; t . super_visit_with (self) ? ; self . universes . pop () ; ControlFlow :: Continue (()) } fn visit_ty (& mut self , ty : I :: Ty) -> Self :: Result { let ty = self . ecx . replace_bound_vars (ty , & mut self . universes) ; let Ok (ty) = self . ecx . structurally_normalize_ty (self . param_env , ty) else { return ControlFlow :: Break (Err (NoSolution)) ; } ; if let ty :: Placeholder (p) = ty . kind () { if p . universe () == ty :: UniverseIndex :: ROOT { ControlFlow :: Break (Ok (())) } else { ControlFlow :: Continue (()) } } else if ty . has_type_flags (TypeFlags :: HAS_PLACEHOLDER | TypeFlags :: HAS_RE_INFER) { ty . super_visit_with (self) } else { ControlFlow :: Continue (()) } } fn visit_const (& mut self , ct : I :: Const) -> Self :: Result { let ct = self . ecx . replace_bound_vars (ct , & mut self . universes) ; let Ok (ct) = self . ecx . structurally_normalize_const (self . param_env , ct) else { return ControlFlow :: Break (Err (NoSolution)) ; } ; if let ty :: ConstKind :: Placeholder (p) = ct . kind () { if p . universe () == ty :: UniverseIndex :: ROOT { ControlFlow :: Break (Ok (())) } else { ControlFlow :: Continue (()) } } else if ct . has_type_flags (TypeFlags :: HAS_PLACEHOLDER | TypeFlags :: HAS_RE_INFER) { ct . super_visit_with (self) } else { ControlFlow :: Continue (()) } } fn visit_region (& mut self , r : I :: Region) -> Self :: Result { match self . ecx . eager_resolve_region (r) . kind () { ty :: ReStatic | ty :: ReError (_) | ty :: ReBound (..) => ControlFlow :: Continue (()) , ty :: RePlaceholder (p) => { if p . universe () == ty :: UniverseIndex :: ROOT { ControlFlow :: Break (Ok (())) } else { ControlFlow :: Continue (()) } } ty :: ReVar (_) => ControlFlow :: Break (Ok (())) , ty :: ReErased | ty :: ReEarlyParam (_) | ty :: ReLateParam (_) => { unreachable ! ("unexpected region in param-env clause") } } } }
    };
}

impl_63!();
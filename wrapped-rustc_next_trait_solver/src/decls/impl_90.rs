macro_rules! deps {
    () => {
        SolverDelegate!();
        ReplaceAliasWithInfer!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < D , I > TypeFolder < I > for ReplaceAliasWithInfer < '_ , '_ , D , I > where D : SolverDelegate < Interner = I > , I : Interner , { fn cx (& self) -> I { self . ecx . cx () } fn fold_ty (& mut self , ty : I :: Ty) -> I :: Ty { match ty . kind () { ty :: Alias (..) if ! ty . has_escaping_bound_vars () => { let infer_ty = self . ecx . next_ty_infer () ; let normalizes_to = ty :: PredicateKind :: AliasRelate (ty . into () , infer_ty . into () , ty :: AliasRelationDirection :: Equate ,) ; self . ecx . add_goal (self . normalization_goal_source , Goal :: new (self . cx () , self . param_env , normalizes_to) ,) ; infer_ty } _ => { if ! ty . has_aliases () { ty } else if let Some (& entry) = self . cache . get (& ty) { return entry ; } else { let res = ty . super_fold_with (self) ; assert ! (self . cache . insert (ty , res) . is_none ()) ; res } } } } fn fold_const (& mut self , ct : I :: Const) -> I :: Const { match ct . kind () { ty :: ConstKind :: Unevaluated (..) if ! ct . has_escaping_bound_vars () => { let infer_ct = self . ecx . next_const_infer () ; let normalizes_to = ty :: PredicateKind :: AliasRelate (ct . into () , infer_ct . into () , ty :: AliasRelationDirection :: Equate ,) ; self . ecx . add_goal (self . normalization_goal_source , Goal :: new (self . cx () , self . param_env , normalizes_to) ,) ; infer_ct } _ => ct . super_fold_with (self) , } } fn fold_predicate (& mut self , predicate : I :: Predicate) -> I :: Predicate { if predicate . allow_normalization () { predicate . super_fold_with (self) } else { predicate } } }
    };
}

impl_90!();
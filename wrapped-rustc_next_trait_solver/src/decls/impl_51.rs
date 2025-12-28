macro_rules! deps {
    () => {
        SolverDelegate!();
        ReplaceProjectionWith!();
        Ambiguous!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < D , I > FallibleTypeFolder < I > for ReplaceProjectionWith < '_ , '_ , I , D > where D : SolverDelegate < Interner = I > , I : Interner , { type Error = Ambiguous ; fn cx (& self) -> I { self . ecx . cx () } fn try_fold_ty (& mut self , ty : I :: Ty) -> Result < I :: Ty , Ambiguous > { if let ty :: Alias (ty :: Projection , alias_ty) = ty . kind () && let Some (term) = self . try_eagerly_replace_alias (alias_ty . into ()) ? { Ok (term . expect_ty ()) } else { ty . try_super_fold_with (self) } } }
    };
}

impl_51!()
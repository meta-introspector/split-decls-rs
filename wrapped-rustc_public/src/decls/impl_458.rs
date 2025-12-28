macro_rules! deps {
    () => {
        Ty!();
        Visitor!();
        Visitable!();
        TyKind!();
        RigidTy!();
    };
}

macro_rules! impl_458 {
    () => {
        deps!();
        impl Visitable for Ty { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { visitor . visit_ty (self) } fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self . kind () { super :: ty :: TyKind :: RigidTy (ty) => ty . visit (visitor) ? , super :: ty :: TyKind :: Alias (_ , alias) => alias . args . visit (visitor) ? , super :: ty :: TyKind :: Param (_) | super :: ty :: TyKind :: Bound (_ , _) => { } } ControlFlow :: Continue (()) } }
    };
}

impl_458!();
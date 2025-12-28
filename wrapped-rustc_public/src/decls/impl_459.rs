macro_rules! deps {
    () => {
        Visitor!();
        TyConstKind!();
        TyConst!();
        Visitable!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl Visitable for TyConst { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { visitor . visit_const (self) } fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match & self . kind { super :: ty :: TyConstKind :: Param (_) | super :: ty :: TyConstKind :: Bound (_ , _) => { } super :: ty :: TyConstKind :: Unevaluated (_ , args) => args . visit (visitor) ? , super :: ty :: TyConstKind :: Value (ty , alloc) => { alloc . visit (visitor) ? ; ty . visit (visitor) ? ; } super :: ty :: TyConstKind :: ZSTValue (ty) => ty . visit (visitor) ? , } ControlFlow :: Continue (()) } }
    };
}

impl_459!();
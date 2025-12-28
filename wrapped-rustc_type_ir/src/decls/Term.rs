macro_rules! deps {
    () => {
        IntoKind!();
        TermKind!();
        Ty!();
        Relate!();
        AliasTerm!();
        ConstKind!();
        TypeFoldable!();
        Const!();
        Interner!();
    };
}

macro_rules! Term {
    () => {
        deps!();
        pub trait Term < I : Interner < Term = Self > > : Copy + Debug + Hash + Eq + IntoKind < Kind = ty :: TermKind < I > > + TypeFoldable < I > + Relate < I > { fn as_type (& self) -> Option < I :: Ty > { if let ty :: TermKind :: Ty (ty) = self . kind () { Some (ty) } else { None } } fn expect_ty (& self) -> I :: Ty { self . as_type () . expect ("expected a type, but found a const") } fn as_const (& self) -> Option < I :: Const > { if let ty :: TermKind :: Const (c) = self . kind () { Some (c) } else { None } } fn expect_const (& self) -> I :: Const { self . as_const () . expect ("expected a const, but found a type") } fn is_infer (self) -> bool { match self . kind () { ty :: TermKind :: Ty (ty) => ty . is_ty_var () , ty :: TermKind :: Const (ct) => ct . is_ct_var () , } } fn is_error (self) -> bool { match self . kind () { ty :: TermKind :: Ty (ty) => ty . is_ty_error () , ty :: TermKind :: Const (ct) => ct . is_ct_error () , } } fn to_alias_term (self) -> Option < ty :: AliasTerm < I > > { match self . kind () { ty :: TermKind :: Ty (ty) => match ty . kind () { ty :: Alias (_kind , alias_ty) => Some (alias_ty . into ()) , _ => None , } , ty :: TermKind :: Const (ct) => match ct . kind () { ty :: ConstKind :: Unevaluated (uv) => Some (uv . into ()) , _ => None , } , } } }
    };
}

Term!();
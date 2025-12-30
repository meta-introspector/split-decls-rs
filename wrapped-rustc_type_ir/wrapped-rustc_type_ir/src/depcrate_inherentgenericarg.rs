// Generated macro for GenericArg (trait)
macro_rules! Depcrate_inherentGenericArg {
() => {
// Module: crate::inherent
// Provides: {"GenericArg"}
// Dependencies: {}
pub trait GenericArg < I : Interner < GenericArg = Self > > : Copy + Debug + Hash + Eq + IntoKind < Kind = ty :: GenericArgKind < I > > + TypeVisitable < I > + Relate < I > + From < I :: Ty > + From < I :: Region > + From < I :: Const > + From < I :: Term > { fn as_term (& self) -> Option < I :: Term > { match self . kind () { ty :: GenericArgKind :: Lifetime (_) => None , ty :: GenericArgKind :: Type (ty) => Some (ty . into ()) , ty :: GenericArgKind :: Const (ct) => Some (ct . into ()) , } } fn as_type (& self) -> Option < I :: Ty > { if let ty :: GenericArgKind :: Type (ty) = self . kind () { Some (ty) } else { None } } fn expect_ty (& self) -> I :: Ty { self . as_type () . expect ("expected a type") } fn as_const (& self) -> Option < I :: Const > { if let ty :: GenericArgKind :: Const (c) = self . kind () { Some (c) } else { None } } fn expect_const (& self) -> I :: Const { self . as_const () . expect ("expected a const") } fn as_region (& self) -> Option < I :: Region > { if let ty :: GenericArgKind :: Lifetime (c) = self . kind () { Some (c) } else { None } } fn expect_region (& self) -> I :: Region { self . as_region () . expect ("expected a const") } fn is_non_region_infer (self) -> bool { match self . kind () { ty :: GenericArgKind :: Lifetime (_) => false , ty :: GenericArgKind :: Type (ty) => ty . is_ty_var () , ty :: GenericArgKind :: Const (ct) => ct . is_ct_var () , } } }
};
}

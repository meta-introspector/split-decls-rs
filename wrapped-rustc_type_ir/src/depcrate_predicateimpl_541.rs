// Generated macro for impl_541 (impl)
macro_rules! Depcrate_predicateimpl_541 {
() => {
// Module: crate::predicate
// Provides: {"impl_541"}
// Dependencies: {}
impl < I : Interner > ty :: Binder < I , ExistentialPredicate < I > > { # [doc = " Given an existential predicate like `?Self: PartialEq<u32>` (e.g., derived from `dyn PartialEq<u32>`),"] # [doc = " and a concrete type `self_ty`, returns a full predicate where the existentially quantified variable `?Self`"] # [doc = " has been replaced with `self_ty` (e.g., `self_ty: PartialEq<u32>`, in our example)."] pub fn with_self_ty (& self , cx : I , self_ty : I :: Ty) -> I :: Clause { match self . skip_binder () { ExistentialPredicate :: Trait (tr) => self . rebind (tr) . with_self_ty (cx , self_ty) . upcast (cx) , ExistentialPredicate :: Projection (p) => { self . rebind (p . with_self_ty (cx , self_ty)) . upcast (cx) } ExistentialPredicate :: AutoTrait (did) => { let generics = cx . generics_of (did . into ()) ; let trait_ref = if generics . count () == 1 { ty :: TraitRef :: new (cx , did , [self_ty]) } else { let err_args = GenericArgs :: extend_with_error (cx , did . into () , & [self_ty . into ()]) ; ty :: TraitRef :: new_from_args (cx , did , err_args) } ; self . rebind (trait_ref) . upcast (cx) } } } }
};
}

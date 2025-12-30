// Generated macro for impl_444 (impl)
macro_rules! Depcrate_foldimpl_444 {
() => {
// Module: crate::fold
// Provides: {"impl_444"}
// Dependencies: {}
impl < I : Interner > TypeFolder < I > for Shifter < I > { fn cx (& self) -> I { self . cx } fn fold_binder < T : TypeFoldable < I > > (& mut self , t : ty :: Binder < I , T >) -> ty :: Binder < I , T > { self . current_index . shift_in (1) ; let t = t . super_fold_with (self) ; self . current_index . shift_out (1) ; t } fn fold_region (& mut self , r : I :: Region) -> I :: Region { match r . kind () { ty :: ReBound (debruijn , br) if debruijn >= self . current_index => { let debruijn = debruijn . shifted_in (self . amount) ; Region :: new_bound (self . cx , debruijn , br) } _ => r , } } fn fold_ty (& mut self , ty : I :: Ty) -> I :: Ty { match ty . kind () { ty :: Bound (debruijn , bound_ty) if debruijn >= self . current_index => { let debruijn = debruijn . shifted_in (self . amount) ; Ty :: new_bound (self . cx , debruijn , bound_ty) } _ if ty . has_vars_bound_at_or_above (self . current_index) => ty . super_fold_with (self) , _ => ty , } } fn fold_const (& mut self , ct : I :: Const) -> I :: Const { match ct . kind () { ty :: ConstKind :: Bound (debruijn , bound_ct) if debruijn >= self . current_index => { let debruijn = debruijn . shifted_in (self . amount) ; Const :: new_bound (self . cx , debruijn , bound_ct) } _ => ct . super_fold_with (self) , } } fn fold_predicate (& mut self , p : I :: Predicate) -> I :: Predicate { if p . has_vars_bound_at_or_above (self . current_index) { p . super_fold_with (self) } else { p } } }
};
}

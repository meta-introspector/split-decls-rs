// Generated macro for impl_366 (impl)
macro_rules! Depcrate_binderimpl_366 {
() => {
// Module: crate::binder
// Provides: {"impl_366"}
// Dependencies: {}
impl < 'a , I : Interner > TypeFolder < I > for ArgFolder < 'a , I > { # [inline] fn cx (& self) -> I { self . cx } fn fold_binder < T : TypeFoldable < I > > (& mut self , t : ty :: Binder < I , T >) -> ty :: Binder < I , T > { self . binders_passed += 1 ; let t = t . super_fold_with (self) ; self . binders_passed -= 1 ; t } fn fold_region (& mut self , r : I :: Region) -> I :: Region { match r . kind () { ty :: ReEarlyParam (data) => { let rk = self . args . get (data . index () as usize) . map (| arg | arg . kind ()) ; match rk { Some (ty :: GenericArgKind :: Lifetime (lt)) => self . shift_region_through_binders (lt) , Some (other) => self . region_param_expected (data , r , other) , None => self . region_param_out_of_range (data , r) , } } ty :: ReBound (..) | ty :: ReLateParam (_) | ty :: ReStatic | ty :: RePlaceholder (_) | ty :: ReErased | ty :: ReError (_) => r , ty :: ReVar (_) => panic ! ("unexpected region: {r:?}") , } } fn fold_ty (& mut self , t : I :: Ty) -> I :: Ty { if ! t . has_param () { return t ; } match t . kind () { ty :: Param (p) => self . ty_for_param (p , t) , _ => t . super_fold_with (self) , } } fn fold_const (& mut self , c : I :: Const) -> I :: Const { if let ty :: ConstKind :: Param (p) = c . kind () { self . const_for_param (p , c) } else { c . super_fold_with (self) } } fn fold_predicate (& mut self , p : I :: Predicate) -> I :: Predicate { if p . has_param () { p . super_fold_with (self) } else { p } } fn fold_clauses (& mut self , c : I :: Clauses) -> I :: Clauses { if c . has_param () { c . super_fold_with (self) } else { c } } }
};
}

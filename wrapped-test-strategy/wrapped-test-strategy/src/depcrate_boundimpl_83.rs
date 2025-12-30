// Generated macro for impl_83 (impl)
macro_rules! Depcrate_boundimpl_83 {
() => {
// Module: crate::bound
// Provides: {"impl_83"}
// Dependencies: {}
impl Bounds { pub fn new (can_extend : bool) -> Self { Bounds { ty : Vec :: new () , pred : Vec :: new () , can_extend , } } pub fn from_data (bound : Option < Vec < Bound > >) -> Self { if let Some (bound) = bound { let mut bs = Self :: new (false) ; for b in bound { bs . push (b) ; } bs } else { Self :: new (true) } } fn push (& mut self , bound : Bound) { match bound { Bound :: Type (ty) => self . ty . push (ty) , Bound :: Predicate (pred) => self . pred . push (pred) , Bound :: Default { .. } => self . can_extend = true , } } pub fn child (& mut self , bound : Option < Vec < Bound > >) -> BoundsChild < '_ > { let bounds = if self . can_extend { Self :: from_data (bound) } else { Self :: new (false) } ; BoundsChild { owner : self , bounds , } } pub fn build_wheres (self , type_param_bounds : TokenStream) -> Vec < WherePredicate > { let mut pred = self . pred ; for ty in self . ty { pred . push (parse_quote ! (# ty : # type_param_bounds)) ; } pred } }
};
}

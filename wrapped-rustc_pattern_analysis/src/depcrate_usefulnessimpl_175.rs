// Generated macro for impl_175 (impl)
macro_rules! Depcrate_usefulnessimpl_175 {
() => {
// Module: crate::usefulness
// Provides: {"impl_175"}
// Dependencies: {}
impl < 'p , Cx : PatCx > PatStack < 'p , Cx > { fn from_pattern (pat : & 'p DeconstructedPat < Cx >) -> Self { PatStack { pats : smallvec ! [PatOrWild :: Pat (pat)] , relevant : true } } fn len (& self) -> usize { self . pats . len () } fn head (& self) -> PatOrWild < 'p , Cx > { self . pats [0] } fn iter (& self) -> impl Iterator < Item = PatOrWild < 'p , Cx > > { self . pats . iter () . copied () } fn expand_or_pat (& self) -> impl Iterator < Item = PatStack < 'p , Cx > > { self . head () . expand_or_pat () . into_iter () . map (move | pat | { let mut new = self . clone () ; new . pats [0] = pat ; new }) } # [doc = " This computes `specialize(ctor, self)`. See top of the file for explanations."] # [doc = " Only call if `ctor.is_covered_by(self.head().ctor())` is true."] fn pop_head_constructor (& self , cx : & Cx , ctor : & Constructor < Cx > , ctor_arity : usize , ctor_is_relevant : bool ,) -> Result < PatStack < 'p , Cx > , Cx :: Error > { let head_pat = self . head () ; if head_pat . as_pat () . is_some_and (| pat | pat . arity () > ctor_arity) { return Err (cx . bug (format_args ! ("uncaught type error: pattern {:?} has inconsistent arity (expected arity <= {ctor_arity})" , head_pat . as_pat () . unwrap ()))) ; } let mut new_pats = head_pat . specialize (ctor , ctor_arity) ; new_pats . extend_from_slice (& self . pats [1 ..]) ; let ctor_is_relevant = ! matches ! (self . head () . ctor () , Constructor :: Wildcard) || ctor_is_relevant ; Ok (PatStack { pats : new_pats , relevant : self . relevant && ctor_is_relevant }) } }
};
}

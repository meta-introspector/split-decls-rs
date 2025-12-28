macro_rules! deps {
    () => {
        PatOrWild!();
        PatCx!();
        DeconstructedPat!();
        Constructor!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > PatOrWild < 'p , Cx > { pub (crate) fn as_pat (& self) -> Option < & 'p DeconstructedPat < Cx > > { match self { PatOrWild :: Wild => None , PatOrWild :: Pat (pat) => Some (pat) , } } pub (crate) fn ctor (self) -> & 'p Constructor < Cx > { match self { PatOrWild :: Wild => & Wildcard , PatOrWild :: Pat (pat) => pat . ctor () , } } pub (crate) fn is_or_pat (& self) -> bool { match self { PatOrWild :: Wild => false , PatOrWild :: Pat (pat) => pat . is_or_pat () , } } # [doc = " Expand this or-pattern into its alternatives. This only expands one or-pattern; use"] # [doc = " `flatten_or_pat` to recursively expand nested or-patterns."] pub (crate) fn expand_or_pat (self) -> SmallVec < [Self ; 1] > { match self { PatOrWild :: Pat (pat) if pat . is_or_pat () => { pat . iter_fields () . map (| ipat | PatOrWild :: Pat (& ipat . pat)) . collect () } _ => smallvec ! [self] , } } # [doc = " Recursively expand this (possibly-nested) or-pattern into its alternatives."] pub (crate) fn flatten_or_pat (self) -> SmallVec < [Self ; 1] > { match self { PatOrWild :: Pat (pat) if pat . is_or_pat () => pat . iter_fields () . flat_map (| ipat | PatOrWild :: Pat (& ipat . pat) . flatten_or_pat ()) . collect () , _ => smallvec ! [self] , } } # [doc = " Specialize this pattern with a constructor."] # [doc = " `other_ctor` can be different from `self.ctor`, but must be covered by it."] pub (crate) fn specialize (& self , other_ctor : & Constructor < Cx > , ctor_arity : usize ,) -> SmallVec < [PatOrWild < 'p , Cx > ; 2] > { match self { PatOrWild :: Wild => (0 .. ctor_arity) . map (| _ | PatOrWild :: Wild) . collect () , PatOrWild :: Pat (pat) => pat . specialize (other_ctor , ctor_arity) , } } }
    };
}

impl_53!();
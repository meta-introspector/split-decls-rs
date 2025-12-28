macro_rules! deps {
    () => {
        PatCx!();
        PatOrWild!();
        PatId!();
        Slice!();
        IndexedPat!();
        DeconstructedPat!();
        SliceKind!();
        Constructor!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < Cx : PatCx > DeconstructedPat < Cx > { pub fn new (ctor : Constructor < Cx > , fields : Vec < IndexedPat < Cx > > , arity : usize , ty : Cx :: Ty , data : Cx :: PatData ,) -> Self { DeconstructedPat { ctor , fields , arity , ty , data , uid : PatId :: new () } } pub fn at_index (self , idx : usize) -> IndexedPat < Cx > { IndexedPat { idx , pat : self } } pub (crate) fn is_or_pat (& self) -> bool { matches ! (self . ctor , Or) } pub fn ctor (& self) -> & Constructor < Cx > { & self . ctor } pub fn ty (& self) -> & Cx :: Ty { & self . ty } # [doc = " Returns the extra data stored in a pattern."] pub fn data (& self) -> & Cx :: PatData { & self . data } pub fn arity (& self) -> usize { self . arity } pub fn iter_fields < 'a > (& 'a self) -> impl Iterator < Item = & 'a IndexedPat < Cx > > { self . fields . iter () } # [doc = " Specialize this pattern with a constructor."] # [doc = " `other_ctor` can be different from `self.ctor`, but must be covered by it."] pub (crate) fn specialize < 'a > (& 'a self , other_ctor : & Constructor < Cx > , other_ctor_arity : usize ,) -> SmallVec < [PatOrWild < 'a , Cx > ; 2] > { if matches ! (other_ctor , PrivateUninhabited) { return smallvec ! [] ; } let mut fields : SmallVec < [_ ; 2] > = (0 .. other_ctor_arity) . map (| _ | PatOrWild :: Wild) . collect () ; match self . ctor { Slice (Slice { kind : SliceKind :: VarLen (prefix , _) , .. }) if self . arity != other_ctor_arity => { for ipat in & self . fields { let new_idx = if ipat . idx < prefix { ipat . idx } else { ipat . idx + other_ctor_arity - self . arity } ; fields [new_idx] = PatOrWild :: Pat (& ipat . pat) ; } } _ => { for ipat in & self . fields { fields [ipat . idx] = PatOrWild :: Pat (& ipat . pat) ; } } } fields } # [doc = " Walk top-down and call `it` in each place where a pattern occurs"] # [doc = " starting with the root pattern `walk` is called on. If `it` returns"] # [doc = " false then we will descend no further but siblings will be processed."] pub fn walk < 'a > (& 'a self , it : & mut impl FnMut (& 'a Self) -> bool) { if ! it (self) { return ; } for p in self . iter_fields () { p . pat . walk (it) } } }
    };
}

impl_45!()
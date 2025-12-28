macro_rules! deps {
    () => {
        PatOrWild!();
        PatCx!();
        Constructor!();
        PatStack!();
        MatrixRow!();
        MatchArm!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > MatrixRow < 'p , Cx > { fn new (arm : & MatchArm < 'p , Cx > , arm_id : usize) -> Self { MatrixRow { pats : PatStack :: from_pattern (arm . pat) , parent_row : arm_id , is_under_guard : arm . has_guard , useful : false , intersects_at_least : DenseBitSet :: new_empty (0) , head_is_branch : true , } } fn len (& self) -> usize { self . pats . len () } fn head (& self) -> PatOrWild < 'p , Cx > { self . pats . head () } fn iter (& self) -> impl Iterator < Item = PatOrWild < 'p , Cx > > { self . pats . iter () } fn expand_or_pat (& self , parent_row : usize) -> impl Iterator < Item = MatrixRow < 'p , Cx > > { let is_or_pat = self . pats . head () . is_or_pat () ; self . pats . expand_or_pat () . map (move | patstack | MatrixRow { pats : patstack , parent_row , is_under_guard : self . is_under_guard , useful : false , intersects_at_least : DenseBitSet :: new_empty (0) , head_is_branch : is_or_pat , }) } # [doc = " This computes `specialize(ctor, self)`. See top of the file for explanations."] # [doc = " Only call if `ctor.is_covered_by(self.head().ctor())` is true."] fn pop_head_constructor (& self , cx : & Cx , ctor : & Constructor < Cx > , ctor_arity : usize , ctor_is_relevant : bool , parent_row : usize ,) -> Result < MatrixRow < 'p , Cx > , Cx :: Error > { Ok (MatrixRow { pats : self . pats . pop_head_constructor (cx , ctor , ctor_arity , ctor_is_relevant) ? , parent_row , is_under_guard : self . is_under_guard , useful : false , intersects_at_least : DenseBitSet :: new_empty (0) , head_is_branch : false , }) } }
    };
}

impl_108!();
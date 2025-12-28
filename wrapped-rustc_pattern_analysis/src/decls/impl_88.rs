macro_rules! deps {
    () => {
        BranchPatUsefulness!();
        PatCx!();
        MatrixRow!();
        Matrix!();
        PatOrWild!();
        RedundancyExplanation!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > BranchPatUsefulness < 'p , Cx > { # [doc = " Update `self` with the usefulness information found in `row`."] fn update (& mut self , row : & MatrixRow < 'p , Cx > , matrix : & Matrix < 'p , Cx >) { self . useful |= row . useful ; for row_id in row . intersects_at_least . iter () { let row = & matrix . rows [row_id] ; if row . useful && ! row . is_under_guard { if let PatOrWild :: Pat (intersecting) = row . head () { self . covered_by . insert (intersecting) ; } } } } # [doc = " Check whether this pattern is redundant, and if so explain why."] fn is_redundant (& self) -> Option < RedundancyExplanation < 'p , Cx > > { if self . useful { None } else { # [cfg_attr (feature = "rustc" , allow (rustc :: potential_query_instability))] let mut covered_by : Vec < _ > = self . covered_by . iter () . copied () . collect () ; covered_by . sort_by_key (| pat | pat . uid) ; Some (RedundancyExplanation { covered_by }) } } }
    };
}

impl_88!()
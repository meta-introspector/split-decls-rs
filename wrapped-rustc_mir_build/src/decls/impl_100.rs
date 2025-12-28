macro_rules! deps {
    () => {
        MatchPairTree!();
        FlatPat!();
        TestCase!();
        Binding!();
        PlaceBase!();
        Candidate!();
        FakeBorrowCollector!();
        SubpatternBindings!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'a , 'b , 'tcx > FakeBorrowCollector < 'a , 'b , 'tcx > { fn fake_borrow (& mut self , place : Place < 'tcx > , kind : FakeBorrowKind) { if self . fake_borrows . get (& place) . is_some_and (| k | * k >= kind) { return ; } self . fake_borrows . insert (place , kind) ; self . fake_borrow_deref_prefixes (place , kind) ; } fn fake_borrow_deref_prefixes (& mut self , place : Place < 'tcx > , kind : FakeBorrowKind) { for (place_ref , elem) in place . as_ref () . iter_projections () . rev () { if let ProjectionElem :: Deref = elem { let place = place_ref . to_place (self . cx . tcx) ; if self . fake_borrows . get (& place) . is_some_and (| k | * k >= kind) { return ; } self . fake_borrows . insert (place , kind) ; } } } fn visit_candidate (& mut self , candidate : & Candidate < 'tcx >) { for binding in & candidate . extra_data . bindings { if let super :: SubpatternBindings :: One (binding) = binding { self . visit_binding (binding) ; } } for match_pair in & candidate . match_pairs { self . visit_match_pair (match_pair) ; } } fn visit_flat_pat (& mut self , flat_pat : & FlatPat < 'tcx >) { for binding in & flat_pat . extra_data . bindings { if let super :: SubpatternBindings :: One (binding) = binding { self . visit_binding (binding) ; } } for match_pair in & flat_pat . match_pairs { self . visit_match_pair (match_pair) ; } } fn visit_match_pair (& mut self , match_pair : & MatchPairTree < 'tcx >) { if let TestCase :: Or { pats , .. } = & match_pair . test_case { for flat_pat in pats . iter () { self . visit_flat_pat (flat_pat) } } else if matches ! (match_pair . test_case , TestCase :: Deref { .. }) { if let Some (place) = match_pair . place { self . fake_borrow (place , FakeBorrowKind :: Deep) ; } } else { if let Some (place) = match_pair . place { self . fake_borrow (place , FakeBorrowKind :: Shallow) ; } for subpair in & match_pair . subpairs { self . visit_match_pair (subpair) ; } } } fn visit_binding (& mut self , Binding { source , .. } : & Binding < 'tcx >) { if let PlaceBase :: Local (l) = self . scrutinee_base && l != source . local { return ; } self . fake_borrow_deref_prefixes (* source , FakeBorrowKind :: Shallow) ; } }
    };
}

impl_100!();
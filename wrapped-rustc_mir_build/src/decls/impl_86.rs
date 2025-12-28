macro_rules! deps {
    () => {
        PatternExtraData!();
        Builder!();
        MatchPairTree!();
        PlaceBuilder!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'a , 'tcx > Builder < 'a , 'tcx > { # [doc = " Builds and pushes [`MatchPairTree`] subtrees, one for each pattern in"] # [doc = " `subpatterns`, representing the fields of a [`PatKind::Variant`] or"] # [doc = " [`PatKind::Leaf`]."] # [doc = ""] # [doc = " Used internally by [`MatchPairTree::for_pattern`]."] fn field_match_pairs (& mut self , match_pairs : & mut Vec < MatchPairTree < 'tcx > > , extra_data : & mut PatternExtraData < 'tcx > , place : PlaceBuilder < 'tcx > , subpatterns : & [FieldPat < 'tcx >] ,) { for fieldpat in subpatterns { let place = place . clone_project (PlaceElem :: Field (fieldpat . field , fieldpat . pattern . ty)) ; MatchPairTree :: for_pattern (place , & fieldpat . pattern , self , match_pairs , extra_data) ; } } # [doc = " Builds [`MatchPairTree`] subtrees for the prefix/middle/suffix parts of an"] # [doc = " array pattern or slice pattern, and adds those trees to `match_pairs`."] # [doc = ""] # [doc = " Used internally by [`MatchPairTree::for_pattern`]."] fn prefix_slice_suffix (& mut self , match_pairs : & mut Vec < MatchPairTree < 'tcx > > , extra_data : & mut PatternExtraData < 'tcx > , place : & PlaceBuilder < 'tcx > , prefix : & [Pat < 'tcx >] , opt_slice : & Option < Box < Pat < 'tcx > > > , suffix : & [Pat < 'tcx >] ,) { let tcx = self . tcx ; let (min_length , exact_size) = if let Some (place_resolved) = place . try_to_place (self) { match place_resolved . ty (& self . local_decls , tcx) . ty . kind () { ty :: Array (_ , length) => (length . try_to_target_usize (tcx) . expect ("expected len of array pat to be definite") , true ,) , _ => ((prefix . len () + suffix . len ()) . try_into () . unwrap () , false) , } } else { ((prefix . len () + suffix . len ()) . try_into () . unwrap () , false) } ; for (idx , subpattern) in prefix . iter () . enumerate () { let elem = ProjectionElem :: ConstantIndex { offset : idx as u64 , min_length , from_end : false } ; let place = place . clone_project (elem) ; MatchPairTree :: for_pattern (place , subpattern , self , match_pairs , extra_data) } if let Some (subslice_pat) = opt_slice { let suffix_len = suffix . len () as u64 ; let subslice = place . clone_project (PlaceElem :: Subslice { from : prefix . len () as u64 , to : if exact_size { min_length - suffix_len } else { suffix_len } , from_end : ! exact_size , }) ; MatchPairTree :: for_pattern (subslice , subslice_pat , self , match_pairs , extra_data) ; } for (idx , subpattern) in suffix . iter () . rev () . enumerate () { let end_offset = (idx + 1) as u64 ; let elem = ProjectionElem :: ConstantIndex { offset : if exact_size { min_length - end_offset } else { end_offset } , min_length , from_end : ! exact_size , } ; let place = place . clone_project (elem) ; MatchPairTree :: for_pattern (place , subpattern , self , match_pairs , extra_data) } } }
    };
}

impl_86!();
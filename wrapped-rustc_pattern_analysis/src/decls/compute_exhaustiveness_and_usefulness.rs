macro_rules! deps {
    () => {
        Matrix!();
        UsefulnessCtxt!();
        Constructor!();
        IntRange!();
        PatOrWild!();
        WitnessMatrix!();
        PatCx!();
        PlaceCtxt!();
    };
}

macro_rules! compute_exhaustiveness_and_usefulness {
    () => {
        deps!();
        # [doc = " The core of the algorithm."] # [doc = ""] # [doc = " This recursively computes witnesses of the non-exhaustiveness of `matrix` (if any). Also tracks"] # [doc = " usefulness of each row in the matrix (in `row.useful`). We track usefulness of subpatterns in"] # [doc = " `mcx.branch_usefulness`."] # [doc = ""] # [doc = " The input `Matrix` and the output `WitnessMatrix` together match the type exhaustively."] # [doc = ""] # [doc = " The key steps are:"] # [doc = " - specialization, where we dig into the rows that have a specific constructor and call ourselves"] # [doc = "     recursively;"] # [doc = " - unspecialization, where we lift the results from the previous step into results for this step"] # [doc = "     (using `apply_constructor` and by updating `row.useful` for each parent row)."] # [doc = " This is all explained at the top of the file."] # [instrument (level = "debug" , skip (mcx) , ret)] fn compute_exhaustiveness_and_usefulness < 'a , 'p , Cx : PatCx > (mcx : & mut UsefulnessCtxt < 'a , 'p , Cx > , matrix : & mut Matrix < 'p , Cx > ,) -> Result < WitnessMatrix < Cx > , Cx :: Error > { debug_assert ! (matrix . rows () . all (| r | r . len () == matrix . column_count ())) ; if ! matrix . wildcard_row_is_relevant && matrix . rows () . all (| r | ! r . pats . relevant) { return Ok (WitnessMatrix :: empty ()) ; } let Some (place) = matrix . head_place () else { mcx . increase_complexity_level (matrix . rows () . len ()) ? ; let mut useful = true ; for (i , row) in matrix . rows_mut () . enumerate () { row . useful = useful ; row . intersects_at_least . insert_range (0 .. i) ; useful &= row . is_under_guard ; } return if useful && matrix . wildcard_row_is_relevant { Ok (WitnessMatrix :: unit_witness ()) } else { Ok (WitnessMatrix :: empty ()) } ; } ; let ctors = matrix . heads () . map (| p | p . ctor ()) ; let (split_ctors , missing_ctors) = place . split_column_ctors (mcx . tycx , ctors) ? ; let ty = & place . ty . clone () ; let pcx = & PlaceCtxt { cx : mcx . tycx , ty } ; let mut ret = WitnessMatrix :: empty () ; for ctor in split_ctors { debug ! ("specialize({:?})" , ctor) ; let ctor_is_relevant = matches ! (ctor , Constructor :: Missing) || missing_ctors . is_empty () || mcx . tycx . exhaustive_witnesses () ; let mut spec_matrix = matrix . specialize_constructor (pcx , & ctor , ctor_is_relevant) ? ; let mut witnesses = ensure_sufficient_stack (| | { compute_exhaustiveness_and_usefulness (mcx , & mut spec_matrix) }) ? ; witnesses . apply_constructor (pcx , & missing_ctors , & ctor) ; ret . extend (witnesses) ; if let Constructor :: IntRange (overlap_range) = ctor { if overlap_range . is_singleton () && spec_matrix . rows . len () >= 2 && spec_matrix . rows . iter () . any (| row | ! row . intersects_at_least . is_empty ()) { collect_overlapping_range_endpoints (mcx . tycx , overlap_range , matrix , & spec_matrix) ; } } matrix . unspecialize (spec_matrix) ; } if missing_ctors . iter () . any (| c | matches ! (c , Constructor :: IntRange (..))) { for missing in & missing_ctors { if let Constructor :: IntRange (gap) = missing { if gap . is_singleton () { collect_non_contiguous_range_endpoints (mcx . tycx , gap , matrix) ; } } } } for row in matrix . rows () { if row . head_is_branch { if let PatOrWild :: Pat (pat) = row . head () { mcx . branch_usefulness . entry (pat . uid) . or_default () . update (row , matrix) ; } } } Ok (ret) }
    };
}

compute_exhaustiveness_and_usefulness!()
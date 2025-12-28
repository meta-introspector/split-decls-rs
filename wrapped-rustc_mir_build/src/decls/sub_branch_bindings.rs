macro_rules! deps {
    () => {
        PatternExtraData!();
        MatchTreeSubBranch!();
        SubpatternBindings!();
        Binding!();
    };
}

macro_rules! sub_branch_bindings {
    () => {
        deps!();
        # [doc = " Collects the bindings for a [`MatchTreeSubBranch`], preserving the order they appear in the"] # [doc = " pattern, as though the or-alternatives chosen in this sub-branch were inlined."] fn sub_branch_bindings < 'tcx > (parents : & [PatternExtraData < 'tcx >] , leaf_bindings : & [SubpatternBindings < 'tcx >] ,) -> Vec < Binding < 'tcx > > { let mut all_bindings = Vec :: with_capacity (leaf_bindings . len ()) ; let mut remainder = parents . iter () . map (| parent | parent . bindings . as_slice ()) . chain ([leaf_bindings]) . filter (| bindings | ! bindings . is_empty ()) ; if let Some (candidate_bindings) = remainder . next () { push_sub_branch_bindings (& mut all_bindings , candidate_bindings , & mut remainder) ; } while let Some (candidate_bindings) = remainder . next () { ty :: tls :: with (| tcx | { tcx . dcx () . delayed_bug ("mismatched or-pattern bindings but no error emitted") }) ; push_sub_branch_bindings (& mut all_bindings , candidate_bindings , & mut remainder) ; } all_bindings }
    };
}

sub_branch_bindings!()
macro_rules! value_assigned_to_local {
    () => {
        # [doc = " If `stmt` is an assignment where the LHS is the given local (with no projections), returns the"] # [doc = " RHS of the assignment."] fn value_assigned_to_local < 'a , 'tcx > (stmt : & 'a mir :: Statement < 'tcx > , local : Local ,) -> Option < & 'a mir :: Rvalue < 'tcx > > { if let mir :: StatementKind :: Assign (box (place , rvalue)) = & stmt . kind && let Some (l) = place . as_local () && local == l { return Some (& * rvalue) ; } None }
    };
}

value_assigned_to_local!()
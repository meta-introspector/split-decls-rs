macro_rules! deps {
    () => {
        Matrix!();
        Constructor!();
        PatOrWild!();
        PatCx!();
        IntRange!();
    };
}

macro_rules! collect_non_contiguous_range_endpoints {
    () => {
        deps!();
        # [doc = " Collect ranges that have a singleton gap between them."] fn collect_non_contiguous_range_endpoints < 'p , Cx : PatCx > (cx : & Cx , gap_range : & IntRange , matrix : & Matrix < 'p , Cx > ,) { let gap = gap_range . lo ; let mut onebefore : SmallVec < [_ ; 1] > = Default :: default () ; let mut oneafter : SmallVec < [_ ; 1] > = Default :: default () ; for pat in matrix . heads () { let PatOrWild :: Pat (pat) = pat else { continue } ; let Constructor :: IntRange (this_range) = pat . ctor () else { continue } ; if gap == this_range . hi { onebefore . push (pat) } else if gap . plus_one () == Some (this_range . lo) { oneafter . push (pat) } } for pat_before in onebefore { cx . lint_non_contiguous_range_endpoints (pat_before , * gap_range , oneafter . as_slice ()) ; } }
    };
}

collect_non_contiguous_range_endpoints!()
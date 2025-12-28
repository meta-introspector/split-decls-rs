macro_rules! deps {
    () => {
        Waiter!();
        QueryJobId!();
        QueryMap!();
    };
}

macro_rules! cycle_check {
    () => {
        deps!();
        # [doc = " Look for query cycles by doing a depth first search starting at `query`."] # [doc = " `span` is the reason for the `query` to execute. This is initially DUMMY_SP."] # [doc = " If a cycle is detected, this initial value is replaced with the span causing"] # [doc = " the cycle."] fn cycle_check < I > (query_map : & QueryMap < I > , query : QueryJobId , span : Span , stack : & mut Vec < (Span , QueryJobId) > , visited : & mut FxHashSet < QueryJobId > ,) -> Option < Option < Waiter > > { if ! visited . insert (query) { return if let Some (p) = stack . iter () . position (| q | q . 1 == query) { stack . drain (0 .. p) ; stack [0] . 0 = span ; Some (None) } else { None } ; } stack . push ((span , query)) ; let r = visit_waiters (query_map , query , | span , successor | { cycle_check (query_map , successor , span , stack , visited) }) ; if r . is_none () { stack . pop () ; } r }
    };
}

cycle_check!();
macro_rules! deps {
    () => {
        CycleHeads!();
        IterationCount!();
        Id!();
        Identity!();
        QueryRevisionsExtraInner!();
        QueryRevisionsExtra!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl QueryRevisionsExtra { pub fn new (# [cfg (feature = "accumulator")] accumulated : AccumulatedMap , mut tracked_struct_ids : ThinVec < (Identity , Id) > , cycle_heads : CycleHeads , iteration : IterationCount ,) -> Self { # [cfg (feature = "accumulator")] let acc = accumulated . is_empty () ; # [cfg (not (feature = "accumulator"))] let acc = true ; let inner = if acc && tracked_struct_ids . is_empty () && cycle_heads . is_empty () && iteration . is_initial () { None } else { tracked_struct_ids . shrink_to_fit () ; Some (Box :: new (QueryRevisionsExtraInner { # [cfg (feature = "accumulator")] accumulated , cycle_heads , tracked_struct_ids , iteration : iteration . into () , cycle_converged : false , })) } ; Self (inner) } }
    };
}

impl_462!()
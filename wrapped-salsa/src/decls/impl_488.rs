macro_rules! deps {
    () => {
        CycleHeads!();
        CompletedQuery!();
        QueryOriginRef!();
        ActiveQueryGuard!();
        Id!();
        Identity!();
        QueryRevisions!();
    };
}

macro_rules! impl_488 {
    () => {
        deps!();
        impl ActiveQueryGuard < '_ > { # [doc = " Initialize the tracked struct ids with the values from the prior execution."] pub (crate) fn seed_tracked_struct_ids (& self , tracked_struct_ids : & [(Identity , Id)]) { unsafe { self . local_state . with_query_stack_unchecked_mut (| stack | { # [cfg (debug_assertions)] assert_eq ! (stack . len () , self . push_len , "mismatched push and pop") ; let frame = stack . last_mut () . unwrap () ; frame . tracked_struct_ids_mut () . seed (tracked_struct_ids) ; }) } } # [doc = " Append the given `outputs` to the query's output list."] pub (crate) fn seed_iteration (& self , previous : & QueryRevisions) { let durability = previous . durability ; let changed_at = previous . changed_at ; let edges = previous . origin . as_ref () . edges () ; let untracked_read = matches ! (previous . origin . as_ref () , QueryOriginRef :: DerivedUntracked (_)) ; let tracked_ids = previous . tracked_struct_ids () ; unsafe { self . local_state . with_query_stack_unchecked_mut (| stack | { # [cfg (debug_assertions)] assert_eq ! (stack . len () , self . push_len , "mismatched push and pop") ; let frame = stack . last_mut () . unwrap () ; frame . seed_iteration (durability , changed_at , edges , untracked_read , tracked_ids) ; }) } } pub (crate) fn take_cycle_heads (& mut self) -> CycleHeads { unsafe { self . local_state . with_query_stack_unchecked_mut (| stack | { # [cfg (debug_assertions)] assert_eq ! (stack . len () , self . push_len) ; let frame = stack . last_mut () . unwrap () ; frame . take_cycle_heads () }) } } # [doc = " Invoked when the query has successfully completed execution."] fn complete (self) -> CompletedQuery { let query = unsafe { self . local_state . with_query_stack_unchecked_mut (| stack | { stack . pop_into_revisions (self . database_key_index , # [cfg (debug_assertions)] self . push_len ,) }) } ; std :: mem :: forget (self) ; query } # [doc = " Pops an active query from the stack. Returns the [`CompletedQuery`]"] # [doc = " which summarizes the other queries that were accessed during this"] # [doc = " query's execution."] # [inline] pub (crate) fn pop (self) -> CompletedQuery { self . complete () } }
    };
}

impl_488!();
macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
        ActiveQuery!();
        CompletedQuery!();
        QueryStack!();
        IterationCount!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl QueryStack { pub (crate) fn push_new_query (& mut self , database_key_index : DatabaseKeyIndex , iteration_count : IterationCount ,) { if self . len < self . stack . len () { self . stack [self . len] . reset_for (database_key_index , iteration_count) ; } else { self . stack . push (ActiveQuery :: new (database_key_index , iteration_count)) ; } self . len += 1 ; } # [cfg (debug_assertions)] pub (crate) fn len (& self) -> usize { self . len } pub (crate) fn pop_into_revisions (& mut self , key : DatabaseKeyIndex , # [cfg (debug_assertions)] push_len : usize ,) -> CompletedQuery { # [cfg (debug_assertions)] assert_eq ! (push_len , self . len () , "unbalanced push/pop") ; debug_assert_ne ! (self . len , 0 , "too many pops") ; self . len -= 1 ; debug_assert_eq ! (self . stack [self . len] . database_key_index , key , "unbalanced push/pop") ; self . stack [self . len] . top_into_revisions () } pub (crate) fn pop (& mut self , key : DatabaseKeyIndex , # [cfg (debug_assertions)] push_len : usize) { # [cfg (debug_assertions)] assert_eq ! (push_len , self . len () , "unbalanced push/pop") ; debug_assert_ne ! (self . len , 0 , "too many pops") ; self . len -= 1 ; debug_assert_eq ! (self . stack [self . len] . database_key_index , key , "unbalanced push/pop") ; self . stack [self . len] . clear () } }
    };
}

impl_18!();
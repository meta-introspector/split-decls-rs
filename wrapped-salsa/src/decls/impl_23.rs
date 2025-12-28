macro_rules! deps {
    () => {
        CapturedQuery!();
        Backtrace!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Backtrace { pub fn capture () -> Option < Self > { crate :: with_attached_database (| db | { db . zalsa_local () . try_with_query_stack (| stack | { Backtrace (stack . iter () . rev () . map (| query | CapturedQuery { database_key_index : query . database_key_index , durability : query . durability , changed_at : query . changed_at , cycle_heads : query . cycle_heads . clone () , iteration_count : query . iteration_count , }) . collect () ,) }) }) ? } }
    };
}

impl_23!();
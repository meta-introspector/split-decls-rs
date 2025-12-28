macro_rules! deps {
    () => {
        Durability!();
        IterationCount!();
        DatabaseKeyIndex!();
        Revision!();
        CycleHeads!();
    };
}

macro_rules! CapturedQuery {
    () => {
        deps!();
        struct CapturedQuery { database_key_index : DatabaseKeyIndex , durability : Durability , changed_at : Revision , cycle_heads : CycleHeads , iteration_count : IterationCount , }
    };
}

CapturedQuery!();
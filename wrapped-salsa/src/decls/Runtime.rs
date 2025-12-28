macro_rules! deps {
    () => {
        Revision!();
        Durability!();
        Table!();
    };
}

macro_rules! Runtime {
    () => {
        deps!();
        # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Runtime { # [doc = " Set to true when the current revision has been canceled."] # [doc = " This is done when we an input is being changed. The flag"] # [doc = " is set back to false once the input has been changed."] # [cfg_attr (feature = "persistence" , serde (skip))] revision_canceled : AtomicBool , # [doc = " Stores the \"last change\" revision for values of each duration."] # [doc = " This vector is always of length at least 1 (for Durability 0)"] # [doc = " but its total length depends on the number of durations. The"] # [doc = " element at index 0 is special as it represents the \"current"] # [doc = " revision\".  In general, we have the invariant that revisions"] # [doc = " in here are *declining* -- that is, `revisions[i] >="] # [doc = " revisions[i + 1]`, for all `i`. This is because when you"] # [doc = " modify a value with durability D, that implies that values"] # [doc = " with durability less than D may have changed too."] revisions : [Revision ; Durability :: LEN] , # [doc = " The dependency graph tracks which runtimes are blocked on one"] # [doc = " another, waiting for queries to terminate."] # [cfg_attr (feature = "persistence" , serde (skip))] dependency_graph : Mutex < DependencyGraph > , # [doc = " Data for instances"] # [cfg_attr (feature = "persistence" , serde (skip))] table : Table , }
    };
}

Runtime!()
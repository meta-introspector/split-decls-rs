macro_rules! deps {
    () => {
        Cycle!();
        DatabaseKeyIndex!();
        AtomicIterationCount!();
        CycleHeads!();
    };
}

macro_rules! CycleHead {
    () => {
        deps!();
        # [doc = " A \"cycle head\" is the query at which we encounter a cycle; that is, if A -> B -> C -> A, then A"] # [doc = " would be the cycle head. It returns an \"initial value\" when the cycle is encountered (if"] # [doc = " fixpoint iteration is enabled for that query), and then is responsible for re-iterating the"] # [doc = " cycle until it converges."] # [derive (Debug)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] pub struct CycleHead { pub (crate) database_key_index : DatabaseKeyIndex , pub (crate) iteration_count : AtomicIterationCount , # [doc = " Marks a cycle head as removed within its `CycleHeads` container."] # [doc = ""] # [doc = " Cycle heads are marked as removed when the memo from the last iteration (a provisional memo)"] # [doc = " is used as the initial value for the next iteration. It's necessary to remove all but its own"] # [doc = " head from the `CycleHeads` container, because the query might now depend on fewer cycles"] # [doc = " (in case of conditional dependencies). However, we can't actually remove the cycle head"] # [doc = " within `fetch_cold_cycle` because we only have a readonly memo. That's what `removed` is used for."] # [cfg_attr (feature = "persistence" , serde (skip))] removed : AtomicBool , }
    };
}

CycleHead!()
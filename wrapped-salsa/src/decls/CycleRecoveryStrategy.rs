macro_rules! deps {
    () => {
        Cycle!();
    };
}

macro_rules! CycleRecoveryStrategy {
    () => {
        deps!();
        # [doc = " Cycle recovery strategy: Is this query capable of recovering from"] # [doc = " a cycle that results from executing the function? If so, how?"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum CycleRecoveryStrategy { # [doc = " Cannot recover from cycles: panic."] # [doc = ""] # [doc = " This is the default."] Panic , # [doc = " Recovers from cycles by fixpoint iterating and/or falling"] # [doc = " back to a sentinel value."] # [doc = ""] # [doc = " This choice is computed by the query's `cycle_recovery`"] # [doc = " function and initial value."] Fixpoint , # [doc = " Recovers from cycles by inserting a fallback value for all"] # [doc = " queries that have a fallback, and ignoring any other query"] # [doc = " in the cycle (as if they were not computed)."] FallbackImmediate , }
    };
}

CycleRecoveryStrategy!()
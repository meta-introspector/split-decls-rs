macro_rules! deps {
    () => {
        Cx!();
    };
}

macro_rules! NestedGoals {
    () => {
        deps!();
        # [doc = " The nested goals of each stack entry and the path from the"] # [doc = " stack entry to that nested goal."] # [doc = ""] # [doc = " They are used when checking whether reevaluating a global cache"] # [doc = " would encounter a cycle or use a provisional cache entry given the"] # [doc = " current search graph state. We need to disable the global cache"] # [doc = " in this case as it could otherwise result in behavioral differences."] # [doc = " Cycles can impact behavior. The cycle ABA may have different final"] # [doc = " results from a the cycle BAB depending on the cycle root."] # [doc = ""] # [doc = " We only start tracking nested goals once we've either encountered"] # [doc = " overflow or a solver cycle. This is a performance optimization to"] # [doc = " avoid tracking nested goals on the happy path."] # [derive_where (Debug , Default , Clone ; X : Cx)] struct NestedGoals < X : Cx > { nested_goals : HashMap < X :: Input , PathsToNested > , }
    };
}

NestedGoals!()
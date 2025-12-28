macro_rules! deps {
    () => {
        CycleHeads!();
        NestedGoals!();
        PathKind!();
        Stack!();
        CandidateHeadUsages!();
        Cx!();
        AvailableDepth!();
        HeadUsages!();
    };
}

macro_rules! StackEntry {
    () => {
        deps!();
        # [doc = " Stack entries of the evaluation stack. Its fields tend to be lazily updated"] # [doc = " when popping a child goal or completely immutable."] # [derive_where (Debug ; X : Cx)] pub (super) struct StackEntry < X : Cx > { pub input : X :: Input , # [doc = " Whether proving this goal is a coinductive step."] # [doc = ""] # [doc = " This is used when encountering a trait solver cycle to"] # [doc = " decide whether the initial provisional result of the cycle."] pub step_kind_from_parent : PathKind , # [doc = " The available depth of a given goal, immutable."] pub available_depth : AvailableDepth , # [doc = " The maximum depth required while evaluating this goal."] pub required_depth : usize , # [doc = " Starts out as `None` and gets set when rerunning this"] # [doc = " goal in case we encounter a cycle."] pub provisional_result : Option < X :: Result > , # [doc = " All cycle heads this goal depends on. Lazily updated and only"] # [doc = " up-to date for the top of the stack."] pub heads : CycleHeads , # [doc = " Whether evaluating this goal encountered overflow. Lazily updated."] pub encountered_overflow : bool , # [doc = " Whether and how this goal has been used as a cycle head. Lazily updated."] pub usages : Option < HeadUsages > , # [doc = " We want to be able to ignore head usages if they happen inside of candidates"] # [doc = " which don't impact the result of a goal. This enables us to avoid rerunning goals"] # [doc = " and is also used when rebasing provisional cache entries."] # [doc = ""] # [doc = " To implement this, we track all usages while evaluating a candidate. If this candidate"] # [doc = " then ends up ignored, we manually remove its usages from `usages` and `heads`."] pub candidate_usages : Option < CandidateHeadUsages > , # [doc = " The nested goals of this goal, see the doc comment of the type."] pub nested_goals : NestedGoals < X > , }
    };
}

StackEntry!();
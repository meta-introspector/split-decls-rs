macro_rules! CurrentGoalKind {
    () => {
        # [doc = " The kind of goal we're currently proving."] # [doc = ""] # [doc = " This has effects on cycle handling handling and on how we compute"] # [doc = " query responses, see the variant descriptions for more info."] # [derive (Debug , Copy , Clone)] enum CurrentGoalKind { Misc , # [doc = " We're proving an trait goal for a coinductive trait, either an auto trait or `Sized`."] # [doc = ""] # [doc = " These are currently the only goals whose impl where-clauses are considered to be"] # [doc = " productive steps."] CoinductiveTrait , # [doc = " Unlike other goals, `NormalizesTo` goals act like functions with the expected term"] # [doc = " always being fully unconstrained. This would weaken inference however, as the nested"] # [doc = " goals never get the inference constraints from the actual normalized-to type."] # [doc = ""] # [doc = " Because of this we return any ambiguous nested goals from `NormalizesTo` to the"] # [doc = " caller when then adds these to its own context. The caller is always an `AliasRelate`"] # [doc = " goal so this never leaks out of the solver."] NormalizesTo , }
    };
}

CurrentGoalKind!()
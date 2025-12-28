macro_rules! deps {
    () => {
        HasChanged!();
        GoalStalledOn!();
    };
}

macro_rules! GoalEvaluation {
    () => {
        deps!();
        # [doc = " The result of evaluating a goal."] pub struct GoalEvaluation < I : Interner > { # [doc = " The goal we've evaluated. This is the input goal, but potentially with its"] # [doc = " inference variables resolved. This never applies any inference constraints"] # [doc = " from evaluating the goal."] # [doc = ""] # [doc = " We rely on this to check whether root goals in HIR typeck had an unresolved"] # [doc = " type inference variable in the input. We must not resolve this after evaluating"] # [doc = " the goal as even if the inference variable has been resolved by evaluating the"] # [doc = " goal itself, this goal may still end up failing due to region uniquification"] # [doc = " later on."] # [doc = ""] # [doc = " This is used as a minor optimization to avoid re-resolving inference variables"] # [doc = " when reevaluating ambiguous goals. E.g. if we've got a goal `?x: Trait` with `?x`"] # [doc = " already being constrained to `Vec<?y>`, then the first evaluation resolves it to"] # [doc = " `Vec<?y>: Trait`. If this goal is still ambiguous and we later resolve `?y` to `u32`,"] # [doc = " then reevaluating this goal now only needs to resolve `?y` while it would otherwise"] # [doc = " have to resolve both `?x` and `?y`,"] pub goal : Goal < I , I :: Predicate > , pub certainty : Certainty , pub has_changed : HasChanged , # [doc = " If the [`Certainty`] was `Maybe`, then keep track of whether the goal has changed"] # [doc = " before rerunning it."] pub stalled_on : Option < GoalStalledOn < I > > , }
    };
}

GoalEvaluation!();
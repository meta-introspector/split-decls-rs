macro_rules! deps {
    () => {
        SolverDelegate!();
        GoalStalledOn!();
        GoalEvaluation!();
    };
}

macro_rules! SolverDelegateEvalExt {
    () => {
        deps!();
        pub trait SolverDelegateEvalExt : SolverDelegate { # [doc = " Evaluates a goal from **outside** of the trait solver."] # [doc = ""] # [doc = " Using this while inside of the solver is wrong as it uses a new"] # [doc = " search graph which would break cycle detection."] fn evaluate_root_goal (& self , goal : Goal < Self :: Interner , < Self :: Interner as Interner > :: Predicate > , span : < Self :: Interner as Interner > :: Span , stalled_on : Option < GoalStalledOn < Self :: Interner > > ,) -> Result < GoalEvaluation < Self :: Interner > , NoSolution > ; # [doc = " Check whether evaluating `goal` with a depth of `root_depth` may"] # [doc = " succeed. This only returns `false` if the goal is guaranteed to"] # [doc = " not hold. In case evaluation overflows and fails with ambiguity this"] # [doc = " returns `true`."] # [doc = ""] # [doc = " This is only intended to be used as a performance optimization"] # [doc = " in coherence checking."] fn root_goal_may_hold_with_depth (& self , root_depth : usize , goal : Goal < Self :: Interner , < Self :: Interner as Interner > :: Predicate > ,) -> bool ; fn evaluate_root_goal_for_proof_tree (& self , goal : Goal < Self :: Interner , < Self :: Interner as Interner > :: Predicate > , span : < Self :: Interner as Interner > :: Span ,) -> (Result < NestedNormalizationGoals < Self :: Interner > , NoSolution > , inspect :: GoalEvaluation < Self :: Interner > ,) ; }
    };
}

SolverDelegateEvalExt!()
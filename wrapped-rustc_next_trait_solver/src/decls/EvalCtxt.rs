macro_rules! deps {
    () => {
        EvaluationStepBuilder!();
        GoalStalledOn!();
        SearchGraph!();
        CurrentGoalKind!();
        SolverDelegate!();
    };
}

macro_rules! EvalCtxt {
    () => {
        deps!();
        pub struct EvalCtxt < 'a , D , I = < D as SolverDelegate > :: Interner > where D : SolverDelegate < Interner = I > , I : Interner , { # [doc = " The inference context that backs (mostly) inference and placeholder terms"] # [doc = " instantiated while solving goals."] # [doc = ""] # [doc = " NOTE: The `InferCtxt` that backs the `EvalCtxt` is intentionally private,"] # [doc = " because the `InferCtxt` is much more general than `EvalCtxt`. Methods such"] # [doc = " as  `take_registered_region_obligations` can mess up query responses,"] # [doc = " using `At::normalize` is totally wrong, calling `evaluate_root_goal` can"] # [doc = " cause coinductive unsoundness, etc."] # [doc = ""] # [doc = " Methods that are generally of use for trait solving are *intentionally*"] # [doc = " re-declared through the `EvalCtxt` below, often with cleaner signatures"] # [doc = " since we don't care about things like `ObligationCause`s and `Span`s here."] # [doc = " If some `InferCtxt` method is missing, please first think defensively about"] # [doc = " the method's compatibility with this solver, or if an existing one does"] # [doc = " the job already."] delegate : & 'a D , # [doc = " The variable info for the `var_values`, only used to make an ambiguous response"] # [doc = " with no constraints."] variables : I :: CanonicalVarKinds , # [doc = " What kind of goal we're currently computing, see the enum definition"] # [doc = " for more info."] current_goal_kind : CurrentGoalKind , pub (super) var_values : CanonicalVarValues < I > , # [doc = " The highest universe index nameable by the caller."] # [doc = ""] # [doc = " When we enter a new binder inside of the query we create new universes"] # [doc = " which the caller cannot name. We have to be careful with variables from"] # [doc = " these new universes when creating the query response."] # [doc = ""] # [doc = " Both because these new universes can prevent us from reaching a fixpoint"] # [doc = " if we have a coinductive cycle and because that's the only way we can return"] # [doc = " new placeholders to the caller."] pub (super) max_input_universe : ty :: UniverseIndex , # [doc = " The opaque types from the canonical input. We only need to return opaque types"] # [doc = " which have been added to the storage while evaluating this goal."] pub (super) initial_opaque_types_storage_num_entries : < D :: Infcx as InferCtxtLike > :: OpaqueTypeStorageEntries , pub (super) search_graph : & 'a mut SearchGraph < D > , nested_goals : Vec < (GoalSource , Goal < I , I :: Predicate > , Option < GoalStalledOn < I > >) > , pub (super) origin_span : I :: Span , tainted : Result < () , NoSolution > , pub (super) inspect : inspect :: EvaluationStepBuilder < D > , }
    };
}

EvalCtxt!()
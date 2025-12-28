macro_rules! deps {
    () => {
        EvalCtxt!();
        ProbeCtxt!();
        SolverDelegate!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < D , I , F , T > ProbeCtxt < '_ , '_ , D , I , F , T > where F : FnOnce (& T) -> inspect :: ProbeKind < I > , D : SolverDelegate < Interner = I > , I : Interner , { pub (in crate :: solve) fn enter_single_candidate (self , f : impl FnOnce (& mut EvalCtxt < '_ , D >) -> T ,) -> (T , CandidateHeadUsages) { self . ecx . search_graph . enter_single_candidate () ; let mut candidate_usages = CandidateHeadUsages :: default () ; let result = self . enter (| ecx | { let result = f (ecx) ; candidate_usages = ecx . search_graph . finish_single_candidate () ; result }) ; (result , candidate_usages) } pub (in crate :: solve) fn enter (self , f : impl FnOnce (& mut EvalCtxt < '_ , D >) -> T) -> T { let ProbeCtxt { ecx : outer , probe_kind , _result } = self ; let delegate = outer . delegate ; let max_input_universe = outer . max_input_universe ; let mut nested = EvalCtxt { delegate , variables : outer . variables , var_values : outer . var_values , current_goal_kind : outer . current_goal_kind , max_input_universe , initial_opaque_types_storage_num_entries : outer . initial_opaque_types_storage_num_entries , search_graph : outer . search_graph , nested_goals : outer . nested_goals . clone () , origin_span : outer . origin_span , tainted : outer . tainted , inspect : outer . inspect . take_and_enter_probe () , } ; let r = nested . delegate . probe (| | { let r = f (& mut nested) ; nested . inspect . probe_final_state (delegate , max_input_universe) ; r }) ; if ! nested . inspect . is_noop () { let probe_kind = probe_kind (& r) ; nested . inspect . probe_kind (probe_kind) ; outer . inspect = nested . inspect . finish_probe () ; } r } }
    };
}

impl_76!();
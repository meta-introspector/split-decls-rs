macro_rules! deps {
    () => {
        DepGraph!();
        DepNode!();
        DepKind!();
        StableHashingContext!();
        FingerprintStyle!();
        DepKindStruct!();
        Deps!();
        MarkFrame!();
    };
}

macro_rules! DepContext {
    () => {
        deps!();
        pub trait DepContext : Copy { type Deps : Deps ; # [doc = " Create a hashing context for hashing new results."] fn with_stable_hashing_context < R > (self , f : impl FnOnce (StableHashingContext < '_ >) -> R) -> R ; # [doc = " Access the DepGraph."] fn dep_graph (& self) -> & DepGraph < Self :: Deps > ; # [doc = " Access the profiler."] fn profiler (& self) -> & SelfProfilerRef ; # [doc = " Access the compiler session."] fn sess (& self) -> & Session ; fn dep_kind_info (& self , dep_node : DepKind) -> & DepKindStruct < Self > ; # [inline (always)] fn fingerprint_style (self , kind : DepKind) -> FingerprintStyle { let data = self . dep_kind_info (kind) ; if data . is_anon { return FingerprintStyle :: Opaque ; } data . fingerprint_style } # [inline (always)] # [doc = " Return whether this kind always require evaluation."] fn is_eval_always (self , kind : DepKind) -> bool { self . dep_kind_info (kind) . is_eval_always } # [doc = " Try to force a dep node to execute and see if it's green."] # [doc = ""] # [doc = " Returns true if the query has actually been forced. It is valid that a query"] # [doc = " fails to be forced, e.g. when the query key cannot be reconstructed from the"] # [doc = " dep-node or when the query kind outright does not support it."] # [inline] # [instrument (skip (self , frame) , level = "debug")] fn try_force_from_dep_node (self , dep_node : DepNode , prev_index : SerializedDepNodeIndex , frame : Option < & MarkFrame < '_ > > ,) -> bool { let cb = self . dep_kind_info (dep_node . kind) ; if let Some (f) = cb . force_from_dep_node { match panic :: catch_unwind (panic :: AssertUnwindSafe (| | f (self , dep_node , prev_index))) { Err (value) => { if ! value . is :: < rustc_errors :: FatalErrorMarker > () { print_markframe_trace (self . dep_graph () , frame) ; } panic :: resume_unwind (value) } Ok (query_has_been_forced) => query_has_been_forced , } } else { false } } # [doc = " Load data from the on-disk cache."] fn try_load_from_on_disk_cache (self , dep_node : DepNode) { let cb = self . dep_kind_info (dep_node . kind) ; if let Some (f) = cb . try_load_from_on_disk_cache { f (self , dep_node) } } fn with_reduced_queries < T > (self , _ : impl FnOnce () -> T) -> T ; }
    };
}

DepContext!();
// Generated macro for incremental_verify_ich_failed (function)
macro_rules! Depcrate_query_plumbingincremental_verify_ich_failed {
() => {
// Module: crate::query::plumbing
// Provides: {"incremental_verify_ich_failed"}
// Dependencies: {}
# [cold] # [inline (never)] fn incremental_verify_ich_failed < Tcx > (tcx : Tcx , prev_index : SerializedDepNodeIndex , result : & dyn Fn () -> String ,) where Tcx : DepContext , { thread_local ! { static INSIDE_VERIFY_PANIC : Cell < bool > = const { Cell :: new (false) } ; } ; let old_in_panic = INSIDE_VERIFY_PANIC . replace (true) ; if old_in_panic { tcx . sess () . dcx () . emit_err (crate :: error :: Reentrant) ; } else { let run_cmd = if let Some (crate_name) = & tcx . sess () . opts . crate_name { format ! ("`cargo clean -p {crate_name}` or `cargo clean`") } else { "`cargo clean`" . to_string () } ; let dep_node = tcx . dep_graph () . data () . unwrap () . prev_node_of (prev_index) ; tcx . sess () . dcx () . emit_err (crate :: error :: IncrementCompilation { run_cmd , dep_node : format ! ("{dep_node:?}") , }) ; panic ! ("Found unstable fingerprints for {dep_node:?}: {}" , result ()) ; } INSIDE_VERIFY_PANIC . set (old_in_panic) ; }
};
}

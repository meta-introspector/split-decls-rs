// Generated macro for C (function)
macro_rules! Depcrate_obligation_forest_testsC {
() => {
// Module: crate::obligation_forest::tests
// Provides: {"C"}
// Dependencies: {}
# [allow (non_snake_case)] fn C < OF , BF , O > (of : OF , bf : BF) -> ClosureObligationProcessor < OF , BF , O , & 'static str > where OF : FnMut (& mut O) -> ProcessResult < O , & 'static str > , BF : FnMut (& [O]) , { ClosureObligationProcessor { process_obligation : of , _process_backedge : bf , marker : PhantomData , } }
};
}

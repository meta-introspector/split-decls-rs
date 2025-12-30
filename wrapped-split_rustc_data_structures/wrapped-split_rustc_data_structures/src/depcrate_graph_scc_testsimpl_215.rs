// Generated macro for impl_215 (impl)
macro_rules! Depcrate_graph_scc_testsimpl_215 {
() => {
// Module: crate::graph::scc::tests
// Provides: {"impl_215"}
// Dependencies: {}
impl Annotations < usize > for MinMaxes { fn new (& self , element : usize) -> MinMaxIn { self . 1 (element) } fn annotate_scc (& mut self , scc : usize , annotation : MinMaxIn) { let i = self . 0 . push (annotation) ; assert ! (i == scc) ; } type Ann = MinMaxIn ; type SccIdx = usize ; }
};
}

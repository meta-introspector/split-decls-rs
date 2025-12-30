// Generated macro for impl_29 (impl)
macro_rules! Depcrate_layout_dfaimpl_29 {
() => {
// Module: crate::layout::dfa
// Provides: {"impl_29"}
// Dependencies: {}
# [doc = " Serialize the DFA using the Graphviz DOT format."] impl < R , T > fmt :: Debug for Dfa < R , T > where R : Region , T : Type , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (f , "digraph {{") ? ; writeln ! (f , "    {:?} [shape = doublecircle]" , self . start) ? ; writeln ! (f , "    {:?} [shape = doublecircle]" , self . accept) ? ; for (src , transitions) in self . transitions . iter () { for (t , dst) in transitions . byte_transitions . iter () { writeln ! (f , "    {src:?} -> {dst:?} [label=\"{t:?}\"]") ? ; } for (t , dst) in transitions . ref_transitions . iter () { writeln ! (f , "    {src:?} -> {dst:?} [label=\"{t:?}\"]") ? ; } } writeln ! (f , "}}") } }
};
}

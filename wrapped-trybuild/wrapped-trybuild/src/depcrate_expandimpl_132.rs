// Generated macro for impl_132 (impl)
macro_rules! Depcrate_expandimpl_132 {
() => {
// Module: crate::expand
// Provides: {"impl_132"}
// Dependencies: {}
impl ExpandedTestSet { fn new () -> Self { ExpandedTestSet { vec : Vec :: new () , path_to_index : Map :: new () , } } fn insert (& mut self , test : Test , error : Option < Error > , is_from_glob : bool) { if let Some (& i) = self . path_to_index . get (& test . path) { let prev = & mut self . vec [i] ; if prev . is_from_glob { prev . test . expected = test . expected ; return ; } } let index = self . vec . len () ; let name = Name (format ! ("trybuild{:03}" , index)) ; self . path_to_index . insert (test . path . clone () , index) ; self . vec . push (ExpandedTest { name , test , error , is_from_glob , }) ; } }
};
}

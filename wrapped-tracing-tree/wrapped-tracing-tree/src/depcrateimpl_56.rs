// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl HierarchicalLayer < fn () -> io :: Stderr > { pub fn new (indent_amount : usize) -> Self { let ansi = io :: stderr () . is_terminal () ; let config = Config { ansi , indent_amount , .. Default :: default () } ; Self { make_writer : io :: stderr , bufs : Mutex :: new (Buffers :: new ()) , config , timer : () , } } }
};
}

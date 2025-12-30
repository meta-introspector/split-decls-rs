// Generated macro for impl_21 (impl)
macro_rules! Depcrate_file_setimpl_21 {
() => {
// Module: crate::file_set
// Provides: {"impl_21"}
// Dependencies: {}
impl fst :: Automaton for PrefixOf < '_ > { type State = usize ; fn start (& self) -> usize { 0 } fn is_match (& self , & state : & usize) -> bool { state != ! 0 } fn can_match (& self , & state : & usize) -> bool { state != ! 0 } fn accept (& self , & state : & usize , byte : u8) -> usize { if self . prefix_of . get (state) == Some (& byte) { state + 1 } else { ! 0 } } }
};
}

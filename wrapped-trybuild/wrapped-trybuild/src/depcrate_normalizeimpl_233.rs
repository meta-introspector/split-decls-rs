// Generated macro for impl_233 (impl)
macro_rules! Depcrate_normalizeimpl_233 {
() => {
// Module: crate::normalize
// Provides: {"impl_233"}
// Dependencies: {}
impl Variations { pub fn preferred (& self) -> & str { self . variations . last () . unwrap () } pub fn any < F : FnMut (& str) -> bool > (& self , mut f : F) -> bool { self . variations . iter () . any (| stderr | f (stderr)) } pub fn concat (& mut self , other : & Self) { for (this , other) in self . variations . iter_mut () . zip (& other . variations) { if ! this . is_empty () && ! other . is_empty () { this . push ('\n') ; } this . push_str (other) ; } } }
};
}

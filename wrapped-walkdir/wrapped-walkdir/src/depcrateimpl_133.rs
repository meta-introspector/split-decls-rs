// Generated macro for impl_133 (impl)
macro_rules! Depcrateimpl_133 {
() => {
// Module: crate
// Provides: {"impl_133"}
// Dependencies: {}
impl < P > Iterator for FilterEntry < IntoIter , P > where P : FnMut (& DirEntry) -> bool , { type Item = Result < DirEntry > ; # [doc = " Advances the iterator and returns the next value."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If the iterator fails to retrieve the next value, this method returns"] # [doc = " an error value. The error will be wrapped in an `Option::Some`."] fn next (& mut self) -> Option < Result < DirEntry > > { loop { let dent = match self . it . next () { None => return None , Some (result) => itry ! (result) , } ; if ! (self . predicate) (& dent) { if dent . is_dir () { self . it . skip_current_dir () ; } continue ; } return Some (Ok (dent)) ; } } }
};
}

// Generated macro for Search (trait)
macro_rules! Depcrate_tablesSearch {
() => {
// Module: crate::tables
// Provides: {"Search"}
// Dependencies: {}
pub trait Search { type T : Clone ; fn search (& self , cp : char) -> Option < Self :: T > ; fn includes (& self , cp : char) -> bool { match self . search (cp) { Some (_) => true , None => false } } }
};
}

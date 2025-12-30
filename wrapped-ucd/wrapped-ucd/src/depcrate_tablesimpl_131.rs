// Generated macro for impl_131 (impl)
macro_rules! Depcrate_tablesimpl_131 {
() => {
// Module: crate::tables
// Provides: {"impl_131"}
// Dependencies: {}
impl Search for [(u16 , u16)] { type T = char ; fn search (& self , cp : char) -> Option < char > { let ca = cp as u32 ; let cb = ca as u16 ; if ca > 65536 { return None ; } match self . binary_search_by (| & (cc , _) | cc . cmp (& cb)) { Ok (idx) => { let (_ , v) = self [idx] ; char :: from_u32 (v as u32) } , _ => None } } }
};
}

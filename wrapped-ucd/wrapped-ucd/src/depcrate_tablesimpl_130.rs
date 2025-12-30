// Generated macro for impl_130 (impl)
macro_rules! Depcrate_tablesimpl_130 {
() => {
// Module: crate::tables
// Provides: {"impl_130"}
// Dependencies: {}
impl < S : Clone > Search for [((u8 , u8 , u8) , S)] { type T = S ; fn search (& self , cp : char) -> Option < S > { let ca = cp as u32 ; match self . binary_search_by (| & ((cb1 , cb2 , cb3) , _) | { let cb : u32 = (cb1 as u32) * 65536 + (cb2 as u32) * 256 + (cb3 as u32) ; cb . cmp (& ca) }) { Ok (idx) => { let (_ , ref v) = self [idx] ; Some (v . clone ()) } , _ => None } } }
};
}

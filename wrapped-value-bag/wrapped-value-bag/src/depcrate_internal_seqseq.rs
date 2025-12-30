// Generated macro for Seq (trait)
macro_rules! Depcrate_internal_seqSeq {
() => {
// Module: crate::internal::seq
// Provides: {"Seq"}
// Dependencies: {}
pub (crate) trait Seq { fn visit (& self , visitor : & mut dyn Visitor < '_ >) ; fn borrowed_visit < 'v > (& 'v self , visitor : & mut dyn Visitor < 'v >) { self . visit (visitor) } }
};
}

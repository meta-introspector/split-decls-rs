// Generated macro for impl_42 (impl)
macro_rules! Depcrate_active_queryimpl_42 {
() => {
// Module: crate::active_query
// Provides: {"impl_42"}
// Dependencies: {}
impl std :: fmt :: Debug for QueryStack { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if f . alternate () { f . debug_list () . entries (self . stack . iter () . map (| q | q . database_key_index)) . finish () } else { f . debug_struct ("QueryStack") . field ("stack" , & self . stack) . field ("len" , & self . len) . finish () } } }
};
}

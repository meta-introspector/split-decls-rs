// Generated macro for impl_48 (impl)
macro_rules! Depcrate_active_queryimpl_48 {
() => {
// Module: crate::active_query
// Provides: {"impl_48"}
// Dependencies: {}
impl fmt :: Debug for CapturedQuery { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut debug_struct = f . debug_struct ("CapturedQuery") ; debug_struct . field ("database_key_index" , & self . database_key_index) . field ("durability" , & self . durability) . field ("changed_at" , & self . changed_at) ; if ! self . cycle_heads . is_empty () { debug_struct . field ("cycle_heads" , & self . cycle_heads) . field ("iteration_count" , & self . iteration_count) ; } debug_struct . finish () } }
};
}

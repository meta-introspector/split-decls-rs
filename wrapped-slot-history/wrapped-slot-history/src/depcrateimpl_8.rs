// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl std :: fmt :: Debug for SlotHistory { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "SlotHistory {{ slot: {} bits:" , self . next_slot) ? ; for i in 0 .. MAX_ENTRIES { if self . bits . get (i) { write ! (f , "1") ? ; } else { write ! (f , "0") ? ; } } Ok (()) } }
};
}

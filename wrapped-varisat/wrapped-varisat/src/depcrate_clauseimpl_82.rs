// Generated macro for impl_82 (impl)
macro_rules! Depcrate_clauseimpl_82 {
() => {
// Module: crate::clause
// Provides: {"impl_82"}
// Dependencies: {}
impl Clause { # [doc = " The clause's header"] pub fn header (& self) -> & ClauseHeader { unsafe { let header_ptr = self . data . as_ptr () as * const ClauseHeader ; & * header_ptr } } # [doc = " Mutable reference to the clause's header"] pub fn header_mut (& mut self) -> & mut ClauseHeader { unsafe { let header_ptr = self . data . as_mut_ptr () as * mut ClauseHeader ; & mut * header_ptr } } # [doc = " The clause's literals"] pub fn lits (& self) -> & [Lit] { unsafe { let lit_ptr = self . data . as_ptr () . add (HEADER_LEN) as * const Lit ; slice :: from_raw_parts (lit_ptr , self . data . len () - HEADER_LEN) } } # [doc = " Mutable slice of the clause's literals"] pub fn lits_mut (& mut self) -> & mut [Lit] { unsafe { let lit_ptr = self . data . as_mut_ptr () . add (HEADER_LEN) as * mut Lit ; slice :: from_raw_parts_mut (lit_ptr , self . data . len () - HEADER_LEN) } } }
};
}

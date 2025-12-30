// Generated macro for impl_113 (impl)
macro_rules! Depcrate_internal_seqimpl_113 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_113"}
// Dependencies: {}
impl < 's , 'f > Slot < 's , 'f > { # [doc = " Fill the slot with a sequence of values."] # [doc = ""] # [doc = " The given value doesn't need to satisfy any particular lifetime constraints."] pub fn fill_seq_slice < I , T > (self , value : & 'f I) -> Result < () , Error > where I : AsRef < [T] > , & 'f T : Into < ValueBag < 'f > > + 'f , { self . fill (| visitor | visitor . seq (SeqSlice :: new_ref (value))) } }
};
}

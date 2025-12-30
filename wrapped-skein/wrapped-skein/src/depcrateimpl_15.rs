// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < N > core :: ops :: BitXor < Block < N > > for Block < N > where N : ArrayLength < u8 > , N : PartialDiv < U8 > , < N as PartialDiv < U8 > > :: Output : ArrayLength < u64 > , N :: ArrayType : Copy , < < N as PartialDiv < U8 > > :: Output as ArrayLength < u64 > > :: ArrayType : Copy , { type Output = Block < N > ; fn bitxor (mut self , rhs : Block < N >) -> Self :: Output { for (s , r) in self . as_word_array_mut () . iter_mut () . zip (rhs . as_word_array ()) { * s ^= * r ; } self } }
};
}

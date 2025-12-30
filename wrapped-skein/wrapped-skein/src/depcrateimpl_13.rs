// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < N > Block < N > where N : ArrayLength < u8 > , N : PartialDiv < U8 > , < N as PartialDiv < U8 > > :: Output : ArrayLength < u64 > , N :: ArrayType : Copy , < < N as PartialDiv < U8 > > :: Output as ArrayLength < u64 > > :: ArrayType : Copy , { fn bytes (& mut self) -> & [u8] { self . as_byte_array () . as_slice () } fn as_byte_array (& self) -> & GenericArray < u8 , N > { unsafe { & self . bytes } } fn as_byte_array_mut (& mut self) -> & mut GenericArray < u8 , N > { unsafe { & mut self . bytes } } fn as_word_array (& self) -> & GenericArray < u64 , < N as PartialDiv < U8 > > :: Output > { unsafe { & self . words } } fn as_word_array_mut (& mut self) -> & mut GenericArray < u64 , < N as PartialDiv < U8 > > :: Output > { unsafe { & mut self . words } } fn from_byte_array (block : & GenericArray < u8 , N >) -> Self { Block { bytes : * block } } }
};
}

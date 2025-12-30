// Generated macro for impl_176 (impl)
macro_rules! Depcrate_arbitraryimpl_176 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_176"}
// Dependencies: {}
# [cfg (feature = "ringbuffer")] impl < 'a , A , const N : usize > Arbitrary < 'a > for RingBuffer < A , N > where A : Arbitrary < 'a > , BitsImpl < N > : Bits , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . take (Self :: CAPACITY) . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . take (Self :: CAPACITY) . collect () } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: recursion_guard (depth , | depth | { let (_ , upper) = A :: size_hint (depth) ; (0 , upper . map (| upper | upper * Self :: CAPACITY)) }) } }
};
}

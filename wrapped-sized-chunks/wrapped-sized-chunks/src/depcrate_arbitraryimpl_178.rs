// Generated macro for impl_178 (impl)
macro_rules! Depcrate_arbitraryimpl_178 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'a , A , T > Arbitrary < 'a > for InlineArray < A , T > where A : Arbitrary < 'a > , T : 'static , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . take (Self :: CAPACITY) . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . take (Self :: CAPACITY) . collect () } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: recursion_guard (depth , | depth | { let (_ , upper) = A :: size_hint (depth) ; (0 , upper . map (| upper | upper * Self :: CAPACITY)) }) } }
};
}

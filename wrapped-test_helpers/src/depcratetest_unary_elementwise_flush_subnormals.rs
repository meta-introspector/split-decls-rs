// Generated macro for test_unary_elementwise_flush_subnormals (function)
macro_rules! Depcratetest_unary_elementwise_flush_subnormals {
() => {
// Module: crate
// Provides: {"test_unary_elementwise_flush_subnormals"}
// Dependencies: {}
# [doc = " Test a unary vector function against a unary scalar function, applied elementwise."] # [doc = ""] # [doc = " Where subnormals are flushed, use approximate equality."] pub fn test_unary_elementwise_flush_subnormals < Scalar , ScalarResult , Vector , VectorResult , const LANES : usize , > (fv : & dyn Fn (Vector) -> VectorResult , fs : & dyn Fn (Scalar) -> ScalarResult , check : & dyn Fn ([Scalar ; LANES]) -> bool ,) where Scalar : Copy + core :: fmt :: Debug + DefaultStrategy + FlushSubnormals , ScalarResult : Copy + biteq :: BitEq + core :: fmt :: Debug + DefaultStrategy + FlushSubnormals , Vector : Into < [Scalar ; LANES] > + From < [Scalar ; LANES] > + Copy , VectorResult : Into < [ScalarResult ; LANES] > + From < [ScalarResult ; LANES] > + Copy , { let flush = | x : Scalar | subnormals :: flush (fs (subnormals :: flush_in (x))) ; test_1 (& | x : [Scalar ; LANES] | { proptest :: prop_assume ! (check (x)) ; let result_v : [ScalarResult ; LANES] = fv (x . into ()) . into () ; let result_s : [ScalarResult ; LANES] = x . iter () . copied () . map (fs) . collect :: < Vec < _ > > () . try_into () . unwrap () ; let result_sf : [ScalarResult ; LANES] = x . iter () . copied () . map (flush) . collect :: < Vec < _ > > () . try_into () . unwrap () ; crate :: prop_assert_biteq ! (result_v , result_s , result_sf) ; Ok (()) }) ; }
};
}

// Generated macro for test_unary_elementwise_approx (function)
macro_rules! Depcratetest_unary_elementwise_approx {
() => {
// Module: crate
// Provides: {"test_unary_elementwise_approx"}
// Dependencies: {}
# [doc = " Test a unary vector function against a unary scalar function, applied elementwise."] # [doc = ""] # [doc = " Floats are checked approximately."] pub fn test_unary_elementwise_approx < Scalar , ScalarResult , Vector , VectorResult , const LANES : usize , > (fv : & dyn Fn (Vector) -> VectorResult , fs : & dyn Fn (Scalar) -> ScalarResult , check : & dyn Fn ([Scalar ; LANES]) -> bool , ulps : i64 ,) where Scalar : Copy + core :: fmt :: Debug + DefaultStrategy , ScalarResult : Copy + approxeq :: ApproxEq + core :: fmt :: Debug + DefaultStrategy , Vector : Into < [Scalar ; LANES] > + From < [Scalar ; LANES] > + Copy , VectorResult : Into < [ScalarResult ; LANES] > + From < [ScalarResult ; LANES] > + Copy , { test_1 (& | x : [Scalar ; LANES] | { proptest :: prop_assume ! (check (x)) ; let result_1 : [ScalarResult ; LANES] = fv (x . into ()) . into () ; let result_2 : [ScalarResult ; LANES] = x . iter () . copied () . map (fs) . collect :: < Vec < _ > > () . try_into () . unwrap () ; crate :: prop_assert_approxeq ! (result_1 , result_2 , ulps) ; Ok (()) }) ; }
};
}

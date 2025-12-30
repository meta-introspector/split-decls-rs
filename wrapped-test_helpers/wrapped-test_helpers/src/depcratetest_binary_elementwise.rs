// Generated macro for test_binary_elementwise (function)
macro_rules! Depcratetest_binary_elementwise {
() => {
// Module: crate
// Provides: {"test_binary_elementwise"}
// Dependencies: {}
# [doc = " Test a binary vector function against a binary scalar function, applied elementwise."] pub fn test_binary_elementwise < Scalar1 , Scalar2 , ScalarResult , Vector1 , Vector2 , VectorResult , const LANES : usize , > (fv : & dyn Fn (Vector1 , Vector2) -> VectorResult , fs : & dyn Fn (Scalar1 , Scalar2) -> ScalarResult , check : & dyn Fn ([Scalar1 ; LANES] , [Scalar2 ; LANES]) -> bool ,) where Scalar1 : Copy + core :: fmt :: Debug + DefaultStrategy , Scalar2 : Copy + core :: fmt :: Debug + DefaultStrategy , ScalarResult : Copy + biteq :: BitEq + core :: fmt :: Debug + DefaultStrategy , Vector1 : Into < [Scalar1 ; LANES] > + From < [Scalar1 ; LANES] > + Copy , Vector2 : Into < [Scalar2 ; LANES] > + From < [Scalar2 ; LANES] > + Copy , VectorResult : Into < [ScalarResult ; LANES] > + From < [ScalarResult ; LANES] > + Copy , { test_2 (& | x : [Scalar1 ; LANES] , y : [Scalar2 ; LANES] | { proptest :: prop_assume ! (check (x , y)) ; let result_1 : [ScalarResult ; LANES] = fv (x . into () , y . into ()) . into () ; let result_2 : [ScalarResult ; LANES] = x . iter () . copied () . zip (y . iter () . copied ()) . map (| (x , y) | fs (x , y)) . collect :: < Vec < _ > > () . try_into () . unwrap () ; crate :: prop_assert_biteq ! (result_1 , result_2) ; Ok (()) }) ; }
};
}

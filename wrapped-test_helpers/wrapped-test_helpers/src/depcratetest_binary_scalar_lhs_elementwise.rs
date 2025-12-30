// Generated macro for test_binary_scalar_lhs_elementwise (function)
macro_rules! Depcratetest_binary_scalar_lhs_elementwise {
() => {
// Module: crate
// Provides: {"test_binary_scalar_lhs_elementwise"}
// Dependencies: {}
# [doc = " Test a binary vector-scalar function against a binary scalar function, applied elementwise."] # [inline (never)] pub fn test_binary_scalar_lhs_elementwise < Scalar1 , Scalar2 , ScalarResult , Vector , VectorResult , const LANES : usize , > (fv : & dyn Fn (Scalar1 , Vector) -> VectorResult , fs : & dyn Fn (Scalar1 , Scalar2) -> ScalarResult , check : & dyn Fn (Scalar1 , [Scalar2 ; LANES]) -> bool ,) where Scalar1 : Copy + Default + core :: fmt :: Debug + DefaultStrategy , Scalar2 : Copy + Default + core :: fmt :: Debug + DefaultStrategy , ScalarResult : Copy + Default + biteq :: BitEq + core :: fmt :: Debug + DefaultStrategy , Vector : Into < [Scalar2 ; LANES] > + From < [Scalar2 ; LANES] > + Copy , VectorResult : Into < [ScalarResult ; LANES] > + From < [ScalarResult ; LANES] > + Copy , { test_2 (& | x : Scalar1 , y : [Scalar2 ; LANES] | { proptest :: prop_assume ! (check (x , y)) ; let result_1 : [ScalarResult ; LANES] = fv (x , y . into ()) . into () ; let result_2 : [ScalarResult ; LANES] = { let mut result = [ScalarResult :: default () ; LANES] ; for (i , o) in y . iter () . zip (result . iter_mut ()) { * o = fs (x , * i) ; } result } ; crate :: prop_assert_biteq ! (result_1 , result_2) ; Ok (()) }) ; }
};
}

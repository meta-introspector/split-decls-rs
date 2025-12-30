// Generated macro for test_binary_scalar_rhs_elementwise (function)
macro_rules! Depcratetest_binary_scalar_rhs_elementwise {
() => {
// Module: crate
// Provides: {"test_binary_scalar_rhs_elementwise"}
// Dependencies: {}
# [doc = " Test a binary vector-scalar function against a binary scalar function, applied elementwise."] # [inline (never)] pub fn test_binary_scalar_rhs_elementwise < Scalar1 , Scalar2 , ScalarResult , Vector , VectorResult , const LANES : usize , > (fv : & dyn Fn (Vector , Scalar2) -> VectorResult , fs : & dyn Fn (Scalar1 , Scalar2) -> ScalarResult , check : & dyn Fn ([Scalar1 ; LANES] , Scalar2) -> bool ,) where Scalar1 : Copy + Default + core :: fmt :: Debug + DefaultStrategy , Scalar2 : Copy + Default + core :: fmt :: Debug + DefaultStrategy , ScalarResult : Copy + Default + biteq :: BitEq + core :: fmt :: Debug + DefaultStrategy , Vector : Into < [Scalar1 ; LANES] > + From < [Scalar1 ; LANES] > + Copy , VectorResult : Into < [ScalarResult ; LANES] > + From < [ScalarResult ; LANES] > + Copy , { test_2 (& | x : [Scalar1 ; LANES] , y : Scalar2 | { proptest :: prop_assume ! (check (x , y)) ; let result_1 : [ScalarResult ; LANES] = fv (x . into () , y) . into () ; let result_2 : [ScalarResult ; LANES] = { let mut result = [ScalarResult :: default () ; LANES] ; for (i , o) in x . iter () . zip (result . iter_mut ()) { * o = fs (* i , y) ; } result } ; crate :: prop_assert_biteq ! (result_1 , result_2) ; Ok (()) }) ; }
};
}

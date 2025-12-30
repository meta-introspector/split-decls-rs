// Generated macro for test_unary_mask_elementwise (function)
macro_rules! Depcratetest_unary_mask_elementwise {
() => {
// Module: crate
// Provides: {"test_unary_mask_elementwise"}
// Dependencies: {}
# [doc = " Test a unary vector function against a unary scalar function, applied elementwise."] # [inline (never)] pub fn test_unary_mask_elementwise < Scalar , Vector , Mask , const LANES : usize > (fv : & dyn Fn (Vector) -> Mask , fs : & dyn Fn (Scalar) -> bool , check : & dyn Fn ([Scalar ; LANES]) -> bool ,) where Scalar : Copy + core :: fmt :: Debug + DefaultStrategy , Vector : Into < [Scalar ; LANES] > + From < [Scalar ; LANES] > + Copy , Mask : Into < [bool ; LANES] > + From < [bool ; LANES] > + Copy , { test_1 (& | x : [Scalar ; LANES] | { proptest :: prop_assume ! (check (x)) ; let result_1 : [bool ; LANES] = fv (x . into ()) . into () ; let result_2 : [bool ; LANES] = { let mut result = [false ; LANES] ; for (i , o) in x . iter () . zip (result . iter_mut ()) { * o = fs (* i) ; } result } ; crate :: prop_assert_biteq ! (result_1 , result_2) ; Ok (()) }) ; }
};
}

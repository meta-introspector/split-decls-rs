// Generated macro for test_binary_mask_elementwise (function)
macro_rules! Depcratetest_binary_mask_elementwise {
() => {
// Module: crate
// Provides: {"test_binary_mask_elementwise"}
// Dependencies: {}
# [doc = " Test a unary vector function against a unary scalar function, applied elementwise."] # [inline (never)] pub fn test_binary_mask_elementwise < Scalar1 , Scalar2 , Vector1 , Vector2 , Mask , const LANES : usize > (fv : & dyn Fn (Vector1 , Vector2) -> Mask , fs : & dyn Fn (Scalar1 , Scalar2) -> bool , check : & dyn Fn ([Scalar1 ; LANES] , [Scalar2 ; LANES]) -> bool ,) where Scalar1 : Copy + core :: fmt :: Debug + DefaultStrategy , Scalar2 : Copy + core :: fmt :: Debug + DefaultStrategy , Vector1 : Into < [Scalar1 ; LANES] > + From < [Scalar1 ; LANES] > + Copy , Vector2 : Into < [Scalar2 ; LANES] > + From < [Scalar2 ; LANES] > + Copy , Mask : Into < [bool ; LANES] > + From < [bool ; LANES] > + Copy , { test_2 (& | x : [Scalar1 ; LANES] , y : [Scalar2 ; LANES] | { proptest :: prop_assume ! (check (x , y)) ; let result_v : [bool ; LANES] = fv (x . into () , y . into ()) . into () ; let result_s : [bool ; LANES] = x . iter () . copied () . zip (y . iter () . copied ()) . map (| (x , y) | fs (x , y)) . collect :: < Vec < _ > > () . try_into () . unwrap () ; crate :: prop_assert_biteq ! (result_v , result_s) ; Ok (()) }) ; }
};
}

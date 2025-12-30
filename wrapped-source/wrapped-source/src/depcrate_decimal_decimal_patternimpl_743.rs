// Generated macro for impl_743 (impl)
macro_rules! Depcrate_decimal_decimal_patternimpl_743 {
() => {
// Module: crate::decimal::decimal_pattern
// Provides: {"impl_743"}
// Dependencies: {}
impl DecimalPattern { pub (crate) fn localize_sign (& self , sign_str : & str) -> (String , String) { let signed_affixes = self . negative . as_ref () . map (| subpattern | (subpattern . prefix . as_str () , subpattern . suffix . as_str ())) . unwrap_or_else (| | ("-" , "")) ; (signed_affixes . 0 . replace ('-' , sign_str) , signed_affixes . 1 . replace ('-' , sign_str) ,) } }
};
}

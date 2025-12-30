// Generated macro for unit_error (macro)
macro_rules! Depcrate_errorunit_error {
() => {
// Module: crate::error
// Provides: {"unit_error"}
// Dependencies: {}
macro_rules ! unit_error { ($ (# [$ docs : meta]) * $ pub : vis $ typ : ident : $ display : literal) => ($ (# [$ docs]) * $ pub struct $ typ { _p : () , } impl :: std :: fmt :: Debug for $ typ { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: fmt :: Result { f . debug_struct (stringify ! ($ typ)) . finish () } } impl :: std :: fmt :: Display for $ typ { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: fmt :: Result { f . write_str ($ display) } } impl :: std :: error :: Error for $ typ { }) }
};
}

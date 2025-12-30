// Generated macro for impl_213 (impl)
macro_rules! Depcrate_testimpl_213 {
() => {
// Module: crate::test
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'v > Visit < 'v > for TestVisit { fn visit_any (& mut self , v : ValueBag) -> Result < () , Error > { panic ! ("unexpected value: {}" , v) } fn visit_i64 (& mut self , v : i64) -> Result < () , Error > { assert_eq ! (self . i64 , v) ; Ok (()) } fn visit_u64 (& mut self , v : u64) -> Result < () , Error > { assert_eq ! (self . u64 , v) ; Ok (()) } fn visit_i128 (& mut self , v : i128) -> Result < () , Error > { assert_eq ! (self . i128 , v) ; Ok (()) } fn visit_u128 (& mut self , v : u128) -> Result < () , Error > { assert_eq ! (self . u128 , v) ; Ok (()) } fn visit_f64 (& mut self , v : f64) -> Result < () , Error > { assert_eq ! (self . f64 , v) ; Ok (()) } fn visit_bool (& mut self , v : bool) -> Result < () , Error > { assert_eq ! (self . bool , v) ; Ok (()) } fn visit_str (& mut self , v : & str) -> Result < () , Error > { assert_eq ! (self . str , v) ; Ok (()) } fn visit_borrowed_str (& mut self , v : & 'v str) -> Result < () , Error > { assert_eq ! (self . borrowed_str , v) ; Ok (()) } fn visit_char (& mut self , v : char) -> Result < () , Error > { assert_eq ! (self . char , v) ; Ok (()) } # [cfg (feature = "error")] fn visit_error (& mut self , err : & (dyn crate :: std :: error :: Error + 'static)) -> Result < () , Error > { assert ! (err . downcast_ref ::< crate :: std :: io :: Error > () . is_some ()) ; Ok (()) } # [cfg (feature = "error")] fn visit_borrowed_error (& mut self , err : & 'v (dyn crate :: std :: error :: Error + 'static) ,) -> Result < () , Error > { assert ! (err . downcast_ref ::< crate :: std :: io :: Error > () . is_some ()) ; Ok (()) } }
};
}

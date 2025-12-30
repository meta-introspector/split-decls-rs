// Generated macro for impl_47 (impl)
macro_rules! Depcrate_fieldimpl_47 {
() => {
// Module: crate::field
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a > From < & 'a dyn Value > for ExpectedValue { fn from (value : & 'a dyn Value) -> Self { struct MockValueBuilder { value : Option < ExpectedValue > , } impl Visit for MockValueBuilder { fn record_f64 (& mut self , _ : & Field , value : f64) { self . value = Some (ExpectedValue :: F64 (value)) ; } fn record_i64 (& mut self , _ : & Field , value : i64) { self . value = Some (ExpectedValue :: I64 (value)) ; } fn record_u64 (& mut self , _ : & Field , value : u64) { self . value = Some (ExpectedValue :: U64 (value)) ; } fn record_bool (& mut self , _ : & Field , value : bool) { self . value = Some (ExpectedValue :: Bool (value)) ; } fn record_str (& mut self , _ : & Field , value : & str) { self . value = Some (ExpectedValue :: Str (value . to_owned ())) ; } fn record_debug (& mut self , _ : & Field , value : & dyn fmt :: Debug) { self . value = Some (ExpectedValue :: Debug (format ! ("{:?}" , value))) ; } } let fake_field = callsite ! (name : "fake" , kind : Kind :: EVENT , fields : fake_field) . metadata () . fields () . field ("fake_field") . unwrap () ; let mut builder = MockValueBuilder { value : None } ; value . record (& fake_field , & mut builder) ; builder . value . expect ("finish called before a value was recorded") } }
};
}

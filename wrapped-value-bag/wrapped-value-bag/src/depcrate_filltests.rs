// Generated macro for tests (module)
macro_rules! Depcrate_filltests {
() => {
// Module: crate::fill
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (target_arch = "wasm32")] use wasm_bindgen_test :: * ; use super :: * ; use crate :: std :: string :: ToString ; # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn fill_value_borrowed () { struct TestFill ; impl Fill for TestFill { fn fill (& self , slot : Slot) -> Result < () , Error > { let dbg = & 1 as & dyn fmt :: Debug ; slot . fill_debug (dbg) } } assert_eq ! ("1" , ValueBag :: from_fill (& TestFill) . to_string ()) ; } # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn fill_cast () { struct TestFill ; impl Fill for TestFill { fn fill (& self , slot : Slot) -> Result < () , Error > { slot . fill_any ("a string") } } assert_eq ! ("a string" , ValueBag :: from_fill (& TestFill) . to_borrowed_str () . expect ("invalid value")) ; } # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn fill_fn_cast () { assert_eq ! (42u64 , ValueBag :: from_fill (&| slot : Slot | slot . fill_any (42u64)) . to_u64 () . unwrap ()) ; } # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn fill_fn_borrowed () { # [derive (Debug)] struct MyValue ; let value = MyValue ; assert_eq ! (format ! ("{:?}" , value) , format ! ("{:?}" , ValueBag :: from_fill (&| slot : Slot | slot . fill_debug (& value)))) ; } }
};
}

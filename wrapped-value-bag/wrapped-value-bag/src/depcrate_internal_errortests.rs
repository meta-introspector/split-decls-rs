// Generated macro for tests (module)
macro_rules! Depcrate_internal_errortests {
() => {
// Module: crate::internal::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [cfg (target_arch = "wasm32")] use wasm_bindgen_test :: * ; use super :: * ; use crate :: { std :: { io , string :: ToString } , test :: * , } ; # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn error_capture () { let err = io :: Error :: from (io :: ErrorKind :: Other) ; assert_eq ! (err . to_string () , ValueBag :: capture_error (& err) . to_borrowed_error () . expect ("invalid value") . to_string ()) ; assert_eq ! (err . to_string () , ValueBag :: from_dyn_error (& err) . to_borrowed_error () . expect ("invalid value") . to_string ()) ; } # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn error_downcast () { let err = io :: Error :: from (io :: ErrorKind :: Other) ; assert ! (ValueBag :: capture_error (& err) . downcast_ref ::< io :: Error > () . is_some ()) ; } # [test] # [cfg_attr (target_arch = "wasm32" , wasm_bindgen_test)] fn error_visit () { let err = io :: Error :: from (io :: ErrorKind :: Other) ; ValueBag :: from_dyn_error (& err) . visit (TestVisit :: default ()) . expect ("failed to visit value") ; } }
};
}

// Generated macro for tests (module)
macro_rules! Depcrate_itertests {
() => {
// Module: crate::iter
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use wasm_bindgen_test :: * ; wasm_bindgen_test_configure ! (run_in_browser) ; # [wasm_bindgen_test] fn it_works () { let map = js_sys :: Map :: new () ; macro_rules ! map_set { ($ key : expr => $ value : expr) => { map . set (& JsValue :: from ($ key) , & JsValue :: from ($ value)) ; } ; } map_set ! ("one" => 1_f64) ; map_set ! ("two" => 2_f64) ; map_set ! ("three" => 3_f64) ; let mut iter = UncheckedIter :: from (map . entries ()) . map (| js_value | { let array = js_sys :: Array :: from (& js_value) ; let array = array . to_vec () ; (array [0] . as_string () . expect_throw ("not string") , array [1] . as_f64 () . expect_throw ("not f64") ,) }) ; assert_eq ! (iter . next () , Some ((String :: from ("one") , 1_f64))) ; assert_eq ! (iter . next () , Some ((String :: from ("two") , 2_f64))) ; assert_eq ! (iter . next () , Some ((String :: from ("three") , 3_f64))) ; assert_eq ! (iter . next () , None) ; } }
};
}

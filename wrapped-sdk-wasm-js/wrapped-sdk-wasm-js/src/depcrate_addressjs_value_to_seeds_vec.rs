// Generated macro for js_value_to_seeds_vec (function)
macro_rules! Depcrate_addressjs_value_to_seeds_vec {
() => {
// Module: crate::address
// Provides: {"js_value_to_seeds_vec"}
// Dependencies: {}
fn js_value_to_seeds_vec (array_of_uint8_arrays : & [JsValue]) -> Result < Vec < Vec < u8 > > , JsValue > { if array_of_uint8_arrays . len () > MAX_SEEDS { return Err (JsValue :: from (std :: format ! ("Too many seeds: {} > {}" , array_of_uint8_arrays . len () , MAX_SEEDS))) ; } array_of_uint8_arrays . iter () . enumerate () . map (| (i , u8_array_js) | { let u8_array = u8_array_js . dyn_ref :: < Uint8Array > () . ok_or_else (| | JsValue :: from (std :: format ! ("Invalid seed type at index {}" , i))) ? ; if u8_array . length () as usize > MAX_SEED_LEN { return Err (JsValue :: from (std :: format ! ("Seed {} too long: {} > {}" , i , u8_array . length () , MAX_SEED_LEN))) ; } Ok (u8_array . to_vec ()) }) . collect :: < Result < Vec < _ > , _ > > () }
};
}

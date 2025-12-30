// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [wasm_bindgen] pub async fn run () -> Result < JsValue , JsValue > { let res = reqwest :: Client :: new () . get ("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master") . header ("Accept" , "application/vnd.github.v3+json") . send () . await ? ; let text = res . text () . await ? ; let branch_info : Branch = serde_json :: from_str (& text) . unwrap () ; Ok (JsValue :: from_serde (& branch_info) . unwrap ()) }
};
}

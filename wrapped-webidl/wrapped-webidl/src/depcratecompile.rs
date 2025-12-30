// Generated macro for compile (function)
macro_rules! Depcratecompile {
() => {
// Module: crate
// Provides: {"compile"}
// Dependencies: {}
# [doc = " Compile the given WebIDL source text into Rust source text containing"] # [doc = " `wasm-bindgen` bindings to the things described in the WebIDL."] pub fn compile (webidl_source : & str , experimental_source : & str , options : Options ,) -> Result < BTreeMap < String , Feature > > { let ast = parse (webidl_source , experimental_source , options) ? ; let features = ast . into_iter () . filter_map (| (name , program) | { let code = program . to_string () ? ; let required_features = program . required_features . into_iter () . collect () ; Some ((name , Feature { required_features , code , } ,)) }) . collect () ; Ok (features) }
};
}

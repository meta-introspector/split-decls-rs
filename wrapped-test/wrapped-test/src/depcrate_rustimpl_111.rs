// Generated macro for impl_111 (impl)
macro_rules! Depcrate_rustimpl_111 {
() => {
// Module: crate::rust
// Provides: {"impl_111"}
// Dependencies: {}
impl Runner < '_ > { fn rustc (& self , edition : Edition) -> Command { let state = self . rust_state . as_ref () . unwrap () ; let opts = & self . opts . rust ; let mut cmd = Command :: new ("rustc") ; cmd . arg (match edition { Edition :: E2021 => "--edition=2021" , Edition :: E2024 => "--edition=2024" , }) . arg (& format ! ("--extern=wit_bindgen={}" , state . wit_bindgen_rlib . display ())) . arg (& format ! ("--extern=futures={}" , state . futures_rlib . display ())) . arg ("--target") . arg (& opts . rust_target) . arg ("-Dwarnings") . arg ("-Cdebuginfo=1") ; for dep in state . wit_bindgen_deps . iter () { cmd . arg (& format ! ("-Ldependency={}" , dep . display ())) ; } cmd } fn produces_component (& self) -> bool { match self . opts . rust . rust_target . as_str () { "wasm32-unknown-unknown" | "wasm32-wasi" | "wasm32-wasip1" => false , _ => true , } } }
};
}

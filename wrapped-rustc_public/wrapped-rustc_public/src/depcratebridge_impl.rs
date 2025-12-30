// Generated macro for bridge_impl (macro)
macro_rules! Depcratebridge_impl {
() => {
// Module: crate
// Provides: {"bridge_impl"}
// Dependencies: {}
macro_rules ! bridge_impl { ($ name : ident , $ ty : ty) => { impl rustc_public_bridge :: bridge ::$ name < compiler_interface :: BridgeTys > for $ ty { fn new (def : crate :: DefId) -> Self { Self (def) } } } ; }
};
}

// Generated macro for make_bridge_trait (macro)
macro_rules! Depcrate_bridgemake_bridge_trait {
() => {
// Module: crate::bridge
// Provides: {"make_bridge_trait"}
// Dependencies: {}
macro_rules ! make_bridge_trait { ($ name : ident) => { pub trait $ name < B : Bridge > { fn new (did : B :: DefId) -> Self ; } } ; }
};
}

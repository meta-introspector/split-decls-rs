// Generated macro for impl_115 (impl)
macro_rules! Depcrate_hyperlinkimpl_115 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_115"}
// Dependencies: {}
impl < T > HyperlinkExt for T { fn link (& self , url : impl ToString) -> PaintedLink < & Self > { PaintedLink { painted : Painted :: new (self) , link : url . to_string () } } }
};
}

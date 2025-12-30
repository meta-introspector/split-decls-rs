// Generated macro for impl_116 (impl)
macro_rules! Depcrate_hyperlinkimpl_116 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_116"}
// Dependencies: {}
# [doc = " Experimental support for hyperlinking."] impl < T > Painted < T > { # [doc = " Create a painted hyperlink with a target URL of `url`."] # [doc = ""] # [doc = " See [`hyperlink`] for details."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::Paint;"] # [doc = " use yansi::hyperlink::HyperlinkExt;"] # [doc = ""] # [doc = " println!(\"See {}.\", \"our docs\".green().link(\"https://docs.rs/yansi\"));"] # [doc = " ```"] pub fn link (& self , url : impl ToString) -> PaintedLink < & Self > { PaintedLink { painted : Painted :: new (self) , link : url . to_string () } } }
};
}

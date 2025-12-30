// Generated macro for HyperlinkExt (trait)
macro_rules! Depcrate_hyperlinkHyperlinkExt {
() => {
// Module: crate::hyperlink
// Provides: {"HyperlinkExt"}
// Dependencies: {}
# [doc = " Extension trait to apply hyperlinks to any value, implemented for all types."] # [doc = ""] # [doc = " See the [module level docs](hyperlink) for usage details."] pub trait HyperlinkExt { # [doc = " Create a painted hyperlink with a target URL of `url`."] # [doc = ""] # [doc = " See [`hyperlink`] for details."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::hyperlink::HyperlinkExt;"] # [doc = ""] # [doc = " println!(\"See {}.\", \"our docs\".link(\"https://docs.rs/yansi\"));"] # [doc = " ```"] fn link (& self , url : impl ToString) -> PaintedLink < & Self > ; }
};
}

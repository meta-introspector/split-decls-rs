// Generated macro for TagParser (trait)
macro_rules! Depcrate_tagTagParser {
() => {
// Module: crate::tag
// Provides: {"TagParser"}
// Dependencies: {}
# [doc = " A type that can parse [`Tag`]s from Tracing events."] # [doc = ""] # [doc = " This trait is blanket-implemented for all `Fn(&tracing::Event) -> Option<Tag>`,"] # [doc = " so top-level `fn`s can be used."] # [doc = ""] # [doc = " See the [module-level documentation](mod@crate::tag) for more details."] pub trait TagParser : 'static { # [doc = " Parse a tag from a [`tracing::Event`]"] fn parse (& self , event : & Event) -> Option < Tag > ; }
};
}

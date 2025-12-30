// Generated macro for scrub_attrs (function)
macro_rules! Depcrate_extensionscrub_attrs {
() => {
// Module: crate::extension
// Provides: {"scrub_attrs"}
// Dependencies: {}
# [doc = " Only keep `#[doc]` attrs."] fn scrub_attrs (attrs : & [Attribute]) -> Vec < Attribute > { attrs . into_iter () . cloned () . filter (| attr | { let ident = & attr . path () . segments [0] . ident ; ident == "doc" || ident == "must_use" }) . collect () }
};
}

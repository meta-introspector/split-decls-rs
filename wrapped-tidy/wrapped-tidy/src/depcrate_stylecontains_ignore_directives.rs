// Generated macro for contains_ignore_directives (function)
macro_rules! Depcrate_stylecontains_ignore_directives {
() => {
// Module: crate::style
// Provides: {"contains_ignore_directives"}
// Dependencies: {}
fn contains_ignore_directives < const N : usize > (path_str : & str , can_contain : bool , contents : & str , checks : [& str ; N] ,) -> [Directive ; N] { let always_ignore_linelength = path_str . contains ("rustdoc-json") ; if ! can_contain && ! always_ignore_linelength { return [Directive :: Deny ; N] ; } checks . map (| check | { if check == LINELENGTH_CHECK && always_ignore_linelength { return Directive :: Ignore (false) ; } if contents . contains (& format ! ("// ignore-tidy-{check}")) || contents . contains (& format ! ("# ignore-tidy-{check}")) || contents . contains (& format ! ("/* ignore-tidy-{check} */")) || contents . contains (& format ! ("<!-- ignore-tidy-{check} -->")) { Directive :: Ignore (false) } else { Directive :: Deny } }) }
};
}

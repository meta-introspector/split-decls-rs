// Generated macro for tests (module)
macro_rules! Depcrate_renderer_for_looptests {
() => {
// Module: crate::renderer::for_loop
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: borrow :: Cow ; use serde_json :: Value ; use super :: ForLoop ; # [test] fn test_that_iterating_on_string_yields_grapheme_clusters () { let text = "a\u{310}e\u{301}o\u{308}\u{332}" . to_string () ; let string = Value :: String (text . clone ()) ; let mut string_loop = ForLoop :: from_string ("whatever" , Cow :: Borrowed (& string)) ; assert_eq ! (* string_loop . get_current_value () , text [0 .. 3]) ; string_loop . increment () ; assert_eq ! (* string_loop . get_current_value () , text [3 .. 6]) ; string_loop . increment () ; assert_eq ! (* string_loop . get_current_value () , text [6 ..]) ; } }
};
}

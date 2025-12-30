// Generated macro for snippet_diff (function)
macro_rules! Depcrate_messagesnippet_diff {
() => {
// Module: crate::message
// Provides: {"snippet_diff"}
// Dependencies: {}
fn snippet_diff (color : Color , content : & str , diff : Option < & Diff >) { fn dotted_line () { println ! ("{}" , "┈" . repeat (60)) ; } term :: color (color) ; dotted_line () ; match diff { Some (diff) => { for chunk in diff . iter (content) { match chunk { Render :: Common (s) => { term :: color (color) ; print ! ("{}" , s) ; } Render :: Unique (s) => { term :: bold_color (color) ; print ! ("\x1B[7m{}" , s) ; } } } } None => print ! ("{}" , content) , } term :: color (color) ; dotted_line () ; term :: reset () ; }
};
}

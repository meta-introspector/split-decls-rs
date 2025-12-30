// Generated macro for cmark_check (function)
macro_rules! Depcratecmark_check {
() => {
// Module: crate
// Provides: {"cmark_check"}
// Dependencies: {}
fn cmark_check (path : & Path , bad : & mut bool , contents : & str) -> Result < () , Box < dyn Error > > { use pulldown_cmark :: { CodeBlockKind , Event , Options , Parser , Tag } ; macro_rules ! cmark_error { ($ bad : expr , $ path : expr , $ range : expr , $ ($ arg : tt) *) => { *$ bad = true ; let lineno = contents [..$ range . start] . chars () . filter (|& ch | ch == '\n') . count () + 1 ; eprint ! ("error in {} (line {}): " , $ path . display () , lineno) ; eprintln ! ("{}" , format_args ! ($ ($ arg) *)) ; } } let options = Options :: all () ; let parser = Parser :: new_ext (contents , options) ; for (event , range) in parser . into_offset_iter () { match event { Event :: Start (Tag :: CodeBlock (CodeBlockKind :: Indented)) => { cmark_error ! (bad , path , range , "indented code blocks should use triple backtick-style \
                    with a language identifier") ; } Event :: Start (Tag :: CodeBlock (CodeBlockKind :: Fenced (languages))) => { if languages . is_empty () { cmark_error ! (bad , path , range , "code block should include an explicit language" ,) ; } } _ => { } } } Ok (()) }
};
}

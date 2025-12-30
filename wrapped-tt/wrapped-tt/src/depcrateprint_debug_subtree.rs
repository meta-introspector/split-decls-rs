// Generated macro for print_debug_subtree (function)
macro_rules! Depcrateprint_debug_subtree {
() => {
// Module: crate
// Provides: {"print_debug_subtree"}
// Dependencies: {}
fn print_debug_subtree < S : fmt :: Debug > (f : & mut fmt :: Formatter < '_ > , subtree : & Subtree < S > , level : usize , iter : TtIter < '_ , S > ,) -> fmt :: Result { let align = "  " . repeat (level) ; let Delimiter { kind , open , close } = & subtree . delimiter ; let delim = match kind { DelimiterKind :: Invisible => "$$" , DelimiterKind :: Parenthesis => "()" , DelimiterKind :: Brace => "{}" , DelimiterKind :: Bracket => "[]" , } ; write ! (f , "{align}SUBTREE {delim} " ,) ? ; write ! (f , "{open:#?}") ? ; write ! (f , " ") ? ; write ! (f , "{close:#?}") ? ; for child in iter { writeln ! (f) ? ; print_debug_token (f , level + 1 , child) ? ; } Ok (()) }
};
}

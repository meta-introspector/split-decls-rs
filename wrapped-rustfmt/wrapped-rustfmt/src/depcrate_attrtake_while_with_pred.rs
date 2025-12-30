// Generated macro for take_while_with_pred (function)
macro_rules! Depcrate_attrtake_while_with_pred {
() => {
// Module: crate::attr
// Provides: {"take_while_with_pred"}
// Dependencies: {}
# [doc = " Returns the first group of attributes that fills the given predicate."] # [doc = " We consider two doc comments are in different group if they are separated by normal comments."] fn take_while_with_pred < 'a , P > (context : & RewriteContext < '_ > , attrs : & 'a [ast :: Attribute] , pred : P ,) -> & 'a [ast :: Attribute] where P : Fn (& ast :: Attribute) -> bool , { let mut len = 0 ; let mut iter = attrs . iter () . peekable () ; while let Some (attr) = iter . next () { if pred (attr) { len += 1 ; } else { break ; } if let Some (next_attr) = iter . peek () { let span_between_attr = mk_sp (attr . span . hi () , next_attr . span . lo ()) ; let snippet = context . snippet (span_between_attr) ; if count_newlines (snippet) >= 2 || snippet . contains ('/') { break ; } } } & attrs [.. len] }
};
}

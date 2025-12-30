// Generated macro for rewrite_initial_doc_comments (function)
macro_rules! Depcrate_attrrewrite_initial_doc_comments {
() => {
// Module: crate::attr
// Provides: {"rewrite_initial_doc_comments"}
// Dependencies: {}
# [doc = " Rewrite the any doc comments which come before any other attributes."] fn rewrite_initial_doc_comments (context : & RewriteContext < '_ > , attrs : & [ast :: Attribute] , shape : Shape ,) -> Result < (usize , Option < String >) , RewriteError > { if attrs . is_empty () { return Ok ((0 , None)) ; } let sugared_docs = take_while_with_pred (context , attrs , | a | a . is_doc_comment ()) ; if ! sugared_docs . is_empty () { let snippet = sugared_docs . iter () . map (| a | context . snippet (a . span)) . collect :: < Vec < _ > > () . join ("\n") ; return Ok ((sugared_docs . len () , Some (rewrite_doc_comment (& snippet , shape . comment (context . config) , context . config ,) ?) ,)) ; } Ok ((0 , None)) }
};
}

// Generated macro for impl_41 (impl)
macro_rules! Depcrate_codegenimpl_41 {
() => {
// Module: crate::codegen
// Provides: {"impl_41"}
// Dependencies: {}
impl CommentBlock { fn extract (tag : & str , text : & str) -> Vec < CommentBlock > { assert ! (tag . starts_with (char :: is_uppercase)) ; let tag = format ! ("{tag}:") ; let mut blocks = CommentBlock :: extract_untagged (text) ; blocks . retain_mut (| block | { let first = block . contents . remove (0) ; let Some (id) = first . strip_prefix (& tag) else { return false ; } ; if block . is_doc { panic ! ("Use plain (non-doc) comments with tags like {tag}:\n    {first}") ; } id . trim () . clone_into (& mut block . id) ; true }) ; blocks } fn extract_untagged (text : & str) -> Vec < CommentBlock > { let mut res = Vec :: new () ; let lines = text . lines () . map (str :: trim_start) ; let dummy_block = CommentBlock { id : String :: new () , line : 0 , contents : Vec :: new () , is_doc : false } ; let mut block = dummy_block . clone () ; for (line_num , line) in lines . enumerate () { match line . strip_prefix ("//") { Some (mut contents) if ! contents . starts_with ('/') => { if let Some ('/' | '!') = contents . chars () . next () { contents = & contents [1 ..] ; block . is_doc = true ; } if let Some (' ') = contents . chars () . next () { contents = & contents [1 ..] ; } block . contents . push (contents . to_owned ()) ; } _ => { if ! block . contents . is_empty () { let block = mem :: replace (& mut block , dummy_block . clone ()) ; res . push (block) ; } block . line = line_num + 2 ; } } } if ! block . contents . is_empty () { res . push (block) ; } res } }
};
}

macro_rules! lex_comment_tag {
    () => {
        # [test] fn lex_comment_tag () { let inputs = vec ! ["{# #comment# {{}} {%%} #}" , "{# #comment# {{}} {%%} #}" , "{#- #comment# {{}} {%%} #}" , "{# #comment# {{}} {%%} -#}" , "{#- #comment# {{}} {%%} -#}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: comment_tag , i) ; } }
    };
}

lex_comment_tag!()
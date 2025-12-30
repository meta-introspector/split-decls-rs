// Generated macro for tokenslice_literals (function)
macro_rules! Depcrate_token_teststokenslice_literals {
() => {
// Module: crate::token::tests
// Provides: {"tokenslice_literals"}
// Dependencies: {}
# [test] fn tokenslice_literals () { type TokenSlice < 'i , 't > = crate :: stream :: TokenSlice < 'i , Token < 't > > ; # [derive (Clone , Debug , PartialEq , Eq)] struct Token < 'i > { kind : TokenKind , raw : & 'i str , } impl PartialEq < & str > for Token < '_ > { fn eq (& self , other : & & str) -> bool { self . raw == * other } } impl PartialEq < TokenKind > for Token < '_ > { fn eq (& self , other : & TokenKind) -> bool { self . kind == * other } } # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum TokenKind { If , LeftParen , RightParen , LeftCurly , RightCurly , Value , } impl < 'i , 't > Parser < TokenSlice < 'i , 't > , & 'i Token < 't > , ErrMode < InputError < TokenSlice < 'i , 't > > > > for TokenKind { fn parse_next (& mut self , input : & mut TokenSlice < 'i , 't > ,) -> TestResult < TokenSlice < 'i , 't > , & 'i Token < 't > > { literal (* self) . parse_next (input) . map (| t | & t [0]) } } let input = [Token { kind : TokenKind :: If , raw : "if" , } , Token { kind : TokenKind :: LeftParen , raw : "(" , } , Token { kind : TokenKind :: Value , raw : "hello" , } , Token { kind : TokenKind :: RightParen , raw : ")" , } , Token { kind : TokenKind :: LeftCurly , raw : "{" , } , Token { kind : TokenKind :: RightCurly , raw : "}" , } ,] ; let mut input = TokenSlice :: new (& input) ; assert_parse ! ((TokenKind :: If , TokenKind :: LeftParen , "hello" , TokenKind :: RightParen) . parse_next (& mut input) , str ! [[r#"
Ok(
    (
        Token {
            kind: If,
            raw: "if",
        },
        Token {
            kind: LeftParen,
            raw: "(",
        },
        [
            Token {
                kind: Value,
                raw: "hello",
            },
        ],
        Token {
            kind: RightParen,
            raw: ")",
        },
    ),
)

"#]] . raw ()) ; }
};
}

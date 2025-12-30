// Generated macro for impl_33 (impl)
macro_rules! Depcrate_parserimpl_33 {
() => {
// Module: crate::parser
// Provides: {"impl_33"}
// Dependencies: {}
impl Parser { fn new (mut tokens : Vec < lexer :: Token >) -> Parser { tokens . reverse () ; Parser { tokens , .. Parser :: default () } } fn peek (& self) -> Option < & lexer :: Token > { self . peek_n (0) } fn peek_n (& self , n : usize) -> Option < & lexer :: Token > { self . tokens . iter () . nth_back (n) } fn bump (& mut self) -> Result < lexer :: Token > { self . tokens . pop () . ok_or_else (| | format_err ! ("unexpected EOF")) } fn expect (& mut self , kind : TokenKind , what : & str) -> Result < () > { let token = self . bump () ? ; if token . kind != kind { bail ! (token . loc , "unexpected token, expected `{}`" , what) ; } Ok (()) } fn is_eof (& self) -> bool { self . tokens . is_empty () } fn finish (self) -> Result < Grammar > { for node_data in & self . grammar . nodes { if matches ! (node_data . rule , DUMMY_RULE) { crate :: error :: bail ! ("Undefined node: {}" , node_data . name) } } Ok (self . grammar) } fn intern_node (& mut self , name : String) -> Node { let len = self . node_table . len () ; let grammar = & mut self . grammar ; * self . node_table . entry (name . clone ()) . or_insert_with (| | { grammar . nodes . push (NodeData { name , rule : DUMMY_RULE }) ; Node (len) }) } fn intern_token (& mut self , name : String) -> Token { let len = self . token_table . len () ; let grammar = & mut self . grammar ; * self . token_table . entry (name . clone ()) . or_insert_with (| | { grammar . tokens . push (TokenData { name }) ; Token (len) }) } }
};
}

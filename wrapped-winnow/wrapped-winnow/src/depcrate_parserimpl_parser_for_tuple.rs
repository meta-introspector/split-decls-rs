// Generated macro for impl_parser_for_tuple (macro)
macro_rules! Depcrate_parserimpl_parser_for_tuple {
() => {
// Module: crate::parser
// Provides: {"impl_parser_for_tuple"}
// Dependencies: {}
macro_rules ! impl_parser_for_tuple { ($ ($ index : tt $ parser : ident $ output : ident) ,+) => (# [allow (non_snake_case)] impl < I : Stream , $ ($ output) ,+, E : ParserError < I >, $ ($ parser) ,+> Parser < I , ($ ($ output) ,+,) , E > for ($ ($ parser) ,+,) where $ ($ parser : Parser < I , $ output , E >) ,+ { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < ($ ($ output) ,+,) , E > { $ (let $ output = self .$ index . parse_next (i) ?;) + Ok (($ ($ output) ,+,)) } }) }
};
}

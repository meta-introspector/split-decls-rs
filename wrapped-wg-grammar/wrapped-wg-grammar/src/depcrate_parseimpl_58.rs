// Generated macro for impl_58 (impl)
macro_rules! Depcrate_parseimpl_58 {
() => {
// Module: crate::parse
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , 'i , I : :: gll :: runtime :: Input < Slice = [:: gll :: proc_macro :: FlatToken] > > Expr < 'a , 'i , I > { pub fn parse_with < R > (input : I , f : impl for < 'b , 'i2 > FnOnce (& 'b :: gll :: runtime :: Parser < 'i2 , _P , _C , I > , ParseResult < 'b , 'i2 , I , Expr < 'b , 'i2 , I > > ,) -> R ,) -> R { :: gll :: runtime :: Parser :: with (input , | mut parser , range | { let call = Call { callee : _C :: Expr , range , } ; parser . threads . spawn (Continuation { code : call . callee , fn_input : call . range , state : 0 , } , call . range ,) ; parse (& mut parser) ; let result = parser . memoizer . longest_result (call) ; f (& parser , result . ok_or (ParseError :: NoParse) . and_then (| range | { let handle = Handle { node : ParseNode { kind : P ! (Expr) , range } , parser : & parser , _marker : PhantomData , } ; if range == call . range { Ok (handle) } else { Err (ParseError :: TooShort (handle)) } })) }) } }
};
}

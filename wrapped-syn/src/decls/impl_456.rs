macro_rules! deps {
    () => {
        Peek!();
        CommaSeparated!();
        Lookahead1!();
        Error!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl < 'a > Lookahead1 < 'a > { # [doc = " Looks at the next token in the parse stream to determine whether it"] # [doc = " matches the requested type of token."] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " Note that this method does not use turbofish syntax. Pass the peek type"] # [doc = " inside of parentheses."] # [doc = ""] # [doc = " - `input.peek(Token![struct])`"] # [doc = " - `input.peek(Token![==])`"] # [doc = " - `input.peek(Ident)`&emsp;*(does not accept keywords)*"] # [doc = " - `input.peek(Ident::peek_any)`"] # [doc = " - `input.peek(Lifetime)`"] # [doc = " - `input.peek(token::Brace)`"] pub fn peek < T : Peek > (& self , token : T) -> bool { let _ = token ; peek_impl (self , T :: Token :: peek , T :: Token :: display) } # [doc = " Triggers an error at the current position of the parse stream."] # [doc = ""] # [doc = " The error message will identify all of the expected token types that"] # [doc = " have been peeked against this lookahead instance."] pub fn error (self) -> Error { let mut comparisons = self . comparisons . into_inner () ; comparisons . retain_mut (| display | { if * display == "`)`" { * display = match self . cursor . scope_delimiter () { Delimiter :: Parenthesis => "`)`" , Delimiter :: Brace => "`}`" , Delimiter :: Bracket => "`]`" , Delimiter :: None => return false , } } true }) ; match comparisons . len () { 0 => { if self . cursor . eof () { Error :: new (self . scope , "unexpected end of input") } else { Error :: new (self . cursor . span () , "unexpected token") } } 1 => { let message = format ! ("expected {}" , comparisons [0]) ; error :: new_at (self . scope , self . cursor , message) } 2 => { let message = format ! ("expected {} or {}" , comparisons [0] , comparisons [1]) ; error :: new_at (self . scope , self . cursor , message) } _ => { let message = format ! ("expected one of: {}" , CommaSeparated (& comparisons)) ; error :: new_at (self . scope , self . cursor , message) } } } }
    };
}

impl_456!();
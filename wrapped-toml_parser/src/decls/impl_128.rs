macro_rules! deps {
    () => {
        TokenKind!();
        Span!();
        Token!();
        Lexer!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl Iterator for Lexer < '_ > { type Item = Token ; fn next (& mut self) -> Option < Self :: Item > { let Some (peek_byte) = self . stream . as_bstr () . first () else { if self . eof { return None ; } else { self . eof = true ; let start = self . stream . current_token_start () ; let span = Span :: new_unchecked (start , start) ; return Some (Token :: new (TokenKind :: Eof , span)) ; } } ; Some (process_token (* peek_byte , & mut self . stream)) } }
    };
}

impl_128!();
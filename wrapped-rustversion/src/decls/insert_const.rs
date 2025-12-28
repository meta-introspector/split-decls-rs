macro_rules! deps {
    () => {
        Qualifiers!();
        Result!();
        Error!();
    };
}

macro_rules! insert_const {
    () => {
        deps!();
        pub (crate) fn insert_const (input : TokenStream , const_span : Span) -> Result < TokenStream > { let ref mut input = crate :: iter :: new (input) ; let mut out = TokenStream :: new () ; let mut qualifiers = Qualifiers :: None ; let mut pending = Vec :: new () ; while let Some (token) = input . next () { match token { TokenTree :: Ident (ref ident) if ident . to_string () == "fn" => { let const_ident = Ident :: new ("const" , const_span) ; out . extend (iter :: once (TokenTree :: Ident (const_ident))) ; out . extend (pending) ; out . extend (iter :: once (token)) ; out . extend (input) ; return Ok (out) ; } TokenTree :: Ident (ref ident) if Qualifiers :: from_ident (ident) > qualifiers => { qualifiers = Qualifiers :: from_ident (ident) ; pending . push (token) ; } TokenTree :: Literal (_) if qualifiers == Qualifiers :: Extern => { qualifiers = Qualifiers :: Abi ; pending . push (token) ; } _ => { qualifiers = Qualifiers :: None ; out . extend (pending . drain (..)) ; out . extend (iter :: once (token)) ; } } } Err (Error :: new (const_span , "only allowed on a fn item")) }
    };
}

insert_const!();
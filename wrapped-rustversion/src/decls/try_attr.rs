macro_rules! deps {
    () => {
        Result!();
        Then!();
        Args!();
    };
}

macro_rules! try_attr {
    () => {
        deps!();
        pub fn try_attr (args : attr :: Args , input : TokenStream) -> Result < TokenStream > { if ! args . condition . eval (crate :: RUSTVERSION) { return Ok (input) ; } let output = match args . then { Then :: Const (const_token) => constfn :: insert_const (input , const_token) ? , Then :: Attribute (then) => { TokenStream :: from_iter (vec ! [TokenTree :: Punct (Punct :: new ('#' , Spacing :: Alone)) , TokenTree :: Group (Group :: new (Delimiter :: Bracket , TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new ("cfg_attr" , Span :: call_site ())) , TokenTree :: Group (Group :: new (Delimiter :: Parenthesis , TokenStream :: from_iter (vec ! [TokenTree :: Ident (Ident :: new ("all" , Span :: call_site ())) , TokenTree :: Group (Group :: new (Delimiter :: Parenthesis , TokenStream :: new () ,)) , TokenTree :: Punct (Punct :: new (',' , Spacing :: Alone)) ,] . into_iter () . chain (then) ,) ,)) ,]) ,)) ,] . into_iter () . chain (input) ,) } } ; Ok (allow_incompatible_msrv (output)) }
    };
}

try_attr!();
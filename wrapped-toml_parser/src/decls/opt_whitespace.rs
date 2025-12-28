macro_rules! deps {
    () => {
        ErrorSink!();
        EventReceiver!();
        TokenKind!();
        Stream!();
    };
}

macro_rules! opt_whitespace {
    () => {
        deps!();
        # [doc = " Parse whitespace, if present"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ws = *wschar"] # [doc = " ```"] fn opt_whitespace (tokens : & mut Stream < '_ > , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { if let Some (ws_token) = next_token_if (tokens , | k | matches ! (k , TokenKind :: Whitespace)) { receiver . whitespace (ws_token . span () , error) ; } }
    };
}

opt_whitespace!()
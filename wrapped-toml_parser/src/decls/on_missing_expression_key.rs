macro_rules! deps {
    () => {
        Token!();
        Expected!();
        ErrorSink!();
        EventReceiver!();
        Stream!();
        ParseError!();
    };
}

macro_rules! on_missing_expression_key {
    () => {
        deps!();
        # [cold] fn on_missing_expression_key (tokens : & mut Stream < '_ > , token : & Token , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { error . report_error (ParseError :: new ("invalid key-value pair") . with_context (token . span ()) . with_expected (& [Expected :: Description ("key")]) . with_unexpected (token . span () . before ()) ,) ; receiver . error (token . span () , error) ; ignore_to_newline (tokens , receiver , error) ; }
    };
}

on_missing_expression_key!()
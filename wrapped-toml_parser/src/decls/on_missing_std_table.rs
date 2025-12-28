macro_rules! deps {
    () => {
        Stream!();
        Token!();
        EventReceiver!();
        ErrorSink!();
        ParseError!();
        Expected!();
    };
}

macro_rules! on_missing_std_table {
    () => {
        deps!();
        # [cold] fn on_missing_std_table (tokens : & mut Stream < '_ > , token : & Token , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { error . report_error (ParseError :: new ("missing table open") . with_context (token . span ()) . with_expected (& [Expected :: Literal ("[")]) . with_unexpected (token . span () . before ()) ,) ; receiver . error (token . span () , error) ; ignore_to_newline (tokens , receiver , error) ; }
    };
}

on_missing_std_table!();
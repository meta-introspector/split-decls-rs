macro_rules! deps {
    () => {
        EventReceiver!();
        Token!();
        Stream!();
        ErrorSink!();
    };
}

macro_rules! on_expression_key_val_sep {
    () => {
        deps!();
        fn on_expression_key_val_sep < 'i > (tokens : & mut Stream < 'i > , eq_token : & 'i Token , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { receiver . key_val_sep (eq_token . span () , error) ; opt_whitespace (tokens , receiver , error) ; value (tokens , receiver , error) ; ws_comment_newline (tokens , receiver , error) ; }
    };
}

on_expression_key_val_sep!();
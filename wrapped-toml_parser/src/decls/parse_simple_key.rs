macro_rules! deps {
    () => {
        ErrorSink!();
        Token!();
        DebugEventReceiver!();
        DebugErrorSink!();
        EventReceiver!();
    };
}

macro_rules! parse_simple_key {
    () => {
        deps!();
        # [doc = " Parse lexed tokens into [`Event`][super::Event]s"] pub fn parse_simple_key (tokens : & [Token] , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { let mut tokens = TokenSlice :: new (tokens) ; # [cfg (feature = "debug")] let mut receiver = DebugEventReceiver :: new (receiver) ; # [cfg (feature = "debug")] let receiver = & mut receiver ; # [cfg (feature = "debug")] let mut error = DebugErrorSink :: new (error) ; # [cfg (feature = "debug")] let error = & mut error ; simple_key (& mut tokens , "invalid key" , receiver , error) ; eof (& mut tokens , receiver , error) ; }
    };
}

parse_simple_key!()
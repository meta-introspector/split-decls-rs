macro_rules! deps {
    () => {
        SpanRange!();
        ThreadBound!();
    };
}

macro_rules! ErrorMessage {
    () => {
        deps!();
        struct ErrorMessage { span : ThreadBound < SpanRange > , message : String , }
    };
}

ErrorMessage!();
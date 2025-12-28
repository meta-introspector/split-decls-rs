macro_rules! deps {
    () => {
        ErrorMessage!();
        Error!();
        ThreadBound!();
        SpanRange!();
    };
}

macro_rules! new2 {
    () => {
        deps!();
        # [cfg (all (feature = "parsing" , any (feature = "full" , feature = "derive")))] pub (crate) fn new2 < T : Display > (start : Span , end : Span , message : T) -> Error { return new2 (start , end , message . to_string ()) ; fn new2 (start : Span , end : Span , message : String) -> Error { Error { messages : vec ! [ErrorMessage { span : ThreadBound :: new (SpanRange { start , end }) , message , }] , } } }
    };
}

new2!()
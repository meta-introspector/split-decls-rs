macro_rules! ParseError {
    () => {
        struct ParseError { span : Span , item_description : String , expected : String , }
    };
}

ParseError!()
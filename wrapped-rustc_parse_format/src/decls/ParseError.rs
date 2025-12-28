macro_rules! deps {
    () => {
        Suggestion!();
    };
}

macro_rules! ParseError {
    () => {
        deps!();
        pub struct ParseError { pub description : String , pub note : Option < String > , pub label : String , pub span : Range < usize > , pub secondary_label : Option < (String , Range < usize >) > , pub suggestion : Suggestion , }
    };
}

ParseError!()
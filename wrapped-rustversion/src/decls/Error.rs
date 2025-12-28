macro_rules! Error {
    () => {
        pub struct Error { begin : Span , end : Span , msg : String , }
    };
}

Error!();
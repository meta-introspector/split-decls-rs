macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! TestOutput {
    () => {
        deps!();
        pub struct TestOutput { pub span : Span , pub kind : Kind , pub content : String , }
    };
}

TestOutput!();
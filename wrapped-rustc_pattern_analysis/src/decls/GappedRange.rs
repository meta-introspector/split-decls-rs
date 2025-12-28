macro_rules! GappedRange {
    () => {
        pub struct GappedRange { pub span : Span , pub gap : String , pub first_range : String , }
    };
}

GappedRange!();
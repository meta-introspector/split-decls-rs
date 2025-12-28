macro_rules! ExpectedMetadata {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Default)] pub (crate) struct ExpectedMetadata { pub (crate) name : Option < String > , pub (crate) level : Option < tracing :: Level > , pub (crate) target : Option < String > , }
    };
}

ExpectedMetadata!()
macro_rules! deps {
    () => {
        ExpectedId!();
        ExpectedMetadata!();
    };
}

macro_rules! ExpectedSpan {
    () => {
        deps!();
        # [doc = " A mock span."] # [doc = ""] # [doc = " This is intended for use with the mock subscriber API in the"] # [doc = " [`subscriber`] module."] # [doc = ""] # [doc = " [`subscriber`]: mod@crate::subscriber"] # [derive (Clone , Default , Eq , PartialEq)] pub struct ExpectedSpan { pub (crate) id : Option < ExpectedId > , pub (crate) metadata : ExpectedMetadata , }
    };
}

ExpectedSpan!();
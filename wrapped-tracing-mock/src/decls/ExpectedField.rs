macro_rules! deps {
    () => {
        ExpectedValue!();
    };
}

macro_rules! ExpectedField {
    () => {
        deps!();
        # [doc = " An expected field."] # [doc = ""] # [doc = " For a detailed description and examples, see the documentation for"] # [doc = " the methods and the [`field`] module."] # [doc = ""] # [doc = " [`field`]: mod@crate::field"] # [derive (Debug)] pub struct ExpectedField { pub (super) name : String , pub (super) value : ExpectedValue , }
    };
}

ExpectedField!();
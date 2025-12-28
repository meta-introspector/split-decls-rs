macro_rules! deps {
    () => {
        ExpectedValue!();
    };
}

macro_rules! ExpectedFields {
    () => {
        deps!();
        # [doc = " An expectation for multiple fields."] # [doc = ""] # [doc = " For a detailed description and examples, see the documentation for"] # [doc = " the methods and the [`field`] module."] # [doc = ""] # [doc = " [`field`]: mod@crate::field"] # [derive (Default , Debug , Eq , PartialEq)] pub struct ExpectedFields { fields : HashMap < String , ExpectedValue > , only : bool , }
    };
}

ExpectedFields!()
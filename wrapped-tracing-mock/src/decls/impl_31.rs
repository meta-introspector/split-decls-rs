macro_rules! deps {
    () => {
        ExpectedFields!();
        ExpectedField!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl From < ExpectedField > for ExpectedFields { fn from (field : ExpectedField) -> Self { ExpectedFields { fields : HashMap :: new () , only : false , } . and (field) } }
    };
}

impl_31!()
macro_rules! deps {
    () => {
        ErrorInner!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Errors that can occur when serializing a type."] # [derive (Clone , PartialEq , Eq)] pub struct Error { pub (crate) inner : ErrorInner , }
    };
}

Error!();
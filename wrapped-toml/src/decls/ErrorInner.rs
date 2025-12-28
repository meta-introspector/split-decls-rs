macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! ErrorInner {
    () => {
        deps!();
        # [doc = " Errors that can occur when deserializing a type."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] # [non_exhaustive] pub (crate) enum ErrorInner { # [doc = " Type could not be serialized to TOML"] UnsupportedType (Option < & 'static str >) , # [doc = " Value was out of range for the given type"] OutOfRange (Option < & 'static str >) , # [doc = " `None` could not be serialized to TOML"] UnsupportedNone , # [doc = " Key was not convertible to `String` for serializing to TOML"] KeyNotString , # [doc = " A serialized date was invalid"] DateInvalid , # [doc = " Other serialization error"] Custom (String) , }
    };
}

ErrorInner!();
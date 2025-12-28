macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " A general error that can occur when working with UUIDs."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct Error (pub (crate) ErrorKind) ;
    };
}

Error!()
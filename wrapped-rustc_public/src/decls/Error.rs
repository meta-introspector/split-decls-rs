macro_rules! Error {
    () => {
        # [doc = " A generic error to represent an API request that cannot be fulfilled."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Error (pub (crate) String) ;
    };
}

Error!();
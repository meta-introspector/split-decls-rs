// Generated macro for Error (enum)
macro_rules! Depcrate_serError {
() => {
// Module: crate::ser
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors returned during serializing to `application/x-www-form-urlencoded`."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Error { Custom (Cow < 'static , str >) , Utf8 (str :: Utf8Error) , }
};
}

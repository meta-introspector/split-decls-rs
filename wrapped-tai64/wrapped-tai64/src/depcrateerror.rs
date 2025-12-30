// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " TAI64 errors."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Error { # [doc = " Invalid length"] LengthInvalid , # [doc = " Nanosecond part must be <= 999999999."] NanosInvalid , }
};
}

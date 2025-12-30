// Generated macro for InvalidUuid (struct)
macro_rules! Depcrate_errorInvalidUuid {
() => {
// Module: crate::error
// Provides: {"InvalidUuid"}
// Dependencies: {}
# [doc = " A string that is guaranteed to fail to parse to a [`Uuid`]."] # [doc = ""] # [doc = " This type acts as a lightweight error indicator, suggesting"] # [doc = " that the string cannot be parsed but offering no error"] # [doc = " details. To get details, use `InvalidUuid::into_err`."] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct InvalidUuid < 'a > (pub (crate) & 'a [u8]) ;
};
}

// Generated macro for Either (enum)
macro_rules! Depcrate_util_eitherEither {
() => {
// Module: crate::util::either
// Provides: {"Either"}
// Dependencies: {}
# [doc = " Combine two different service types into a single type."] # [doc = ""] # [doc = " Both services must be of the same request, response, and error types."] # [doc = " [`Either`] is useful for handling conditional branching in service middleware"] # [doc = " to different inner service types."] # [derive (Clone , Copy , Debug)] pub enum Either < A , B > { # [allow (missing_docs)] Left (A) , # [allow (missing_docs)] Right (B) , }
};
}

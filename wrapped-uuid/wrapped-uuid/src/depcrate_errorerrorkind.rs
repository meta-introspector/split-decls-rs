// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_errorErrorKind {
() => {
// Module: crate::error
// Provides: {"ErrorKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , Hash , PartialEq)] pub (crate) enum ErrorKind { # [doc = " Invalid character in the [`Uuid`] string."] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] ParseChar { character : char , index : usize } , # [doc = " A simple [`Uuid`] didn't contain 32 characters."] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] ParseSimpleLength { len : usize } , # [doc = " A byte array didn't contain 16 bytes"] ParseByteLength { len : usize } , # [doc = " A hyphenated [`Uuid`] didn't contain 5 groups"] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] ParseGroupCount { count : usize } , # [doc = " A hyphenated [`Uuid`] had a group that wasn't the right length"] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] ParseGroupLength { group : usize , len : usize , index : usize , } , # [doc = " The input was not a valid UTF8 string"] ParseInvalidUTF8 , # [doc = " Some other parsing error occurred."] ParseOther , # [doc = " The UUID is nil."] Nil , # [doc = " A system time was invalid."] # [cfg (feature = "std")] InvalidSystemTime (& 'static str) , }
};
}

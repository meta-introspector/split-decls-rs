// Generated macro for AsyncReadBody (type)
macro_rules! Depcrate_compression_utilsAsyncReadBody {
() => {
// Module: crate::compression_utils
// Provides: {"AsyncReadBody"}
// Dependencies: {}
# [doc = " A `Body` that has been converted into an `AsyncRead`."] pub (crate) type AsyncReadBody < B > = StreamReader < StreamErrorIntoIoError < BodyIntoStream < B > , < B as Body > :: Error > , < B as Body > :: Data > ;
};
}

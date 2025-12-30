// Generated macro for StorageError (enum)
macro_rules! Depcrate_errorsStorageError {
() => {
// Module: crate::errors
// Provides: {"StorageError"}
// Dependencies: {}
# [doc = " Error returned by this crate"] # [derive (Debug , thiserror :: Error)] pub enum StorageError { # [doc = " Error from `serde`"] # [error ("{0}")] SerdeError (# [from] serde_json :: Error) , # [doc = " Error if the requested key is not found"] # [error ("key {0} not found")] KeyNotFound (String) , # [doc = " Error returned from JavaScript"] # [error ("{0}")] JsError (JsError) , }
};
}

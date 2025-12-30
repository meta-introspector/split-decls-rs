// Generated macro for form (function)
macro_rules! Depcrate_filters_multipartform {
() => {
// Module: crate::filters::multipart
// Provides: {"form"}
// Dependencies: {}
# [doc = " Create a [`Filter`](crate::Filter) to extract a `multipart/form-data` body from a request."] # [doc = ""] # [doc = " The extracted `FormData` type is a `Stream` of `Part`s, and each `Part`"] # [doc = " in turn is a `Stream` of bytes."] pub fn form () -> FormOptions { FormOptions { max_length : Some (DEFAULT_FORM_DATA_MAX_LENGTH) , } }
};
}

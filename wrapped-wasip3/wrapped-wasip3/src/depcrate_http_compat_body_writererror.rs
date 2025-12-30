// Generated macro for Error (enum)
macro_rules! Depcrate_http_compat_body_writerError {
() => {
// Module: crate::http_compat::body_writer
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug , thiserror :: Error)] pub enum Error { # [doc = " The [`http_body::Body`] returned an error."] # [error ("body error: {0}")] HttpBody (# [source] BoxError) , # [doc = " Received trailers were rejected by [`Trailers::from_list`]."] # [error ("invalid trailers: {0}")] InvalidTrailers (# [source] HeaderError) , # [doc = " The result future reader end was closed (dropped)."] # [doc = ""] # [doc = " The result that couldn't be written is returned."] # [error ("result future reader closed")] ResultReaderClosed (BodyResult) , # [doc = " The stream reader end was closed (dropped)."] # [doc = ""] # [doc = " The number of bytes written successfully is returned as `written` and"] # [doc = " the bytes that couldn't be written are returned as `unwritten`."] # [error ("stream reader closed")] StreamReaderClosed { written : usize , unwritten : Vec < u8 > } , }
};
}

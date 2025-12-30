// Generated macro for BodyWriter (struct)
macro_rules! Depcrate_http_compat_body_writerBodyWriter {
() => {
// Module: crate::http_compat::body_writer
// Provides: {"BodyWriter"}
// Dependencies: {}
# [doc = " BodyWriter coordinates a [`StreamWriter`] and [`FutureWriter`] associated"] # [doc = " with the write end of a `wasi:http` `Request` or `Response` body."] pub struct BodyWriter { pub stream_writer : StreamWriter < u8 > , pub result_writer : FutureWriter < BodyResult > , pub trailers : HeaderMap , }
};
}

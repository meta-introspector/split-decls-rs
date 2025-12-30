// Generated macro for body_into_text (function)
macro_rules! Depcrate_services_fs_serve_dir_testsbody_into_text {
() => {
// Module: crate::services::fs::serve_dir::tests
// Provides: {"body_into_text"}
// Dependencies: {}
async fn body_into_text < B > (body : B) -> String where B : HttpBody < Data = bytes :: Bytes > + Unpin , B :: Error : std :: fmt :: Debug , { let bytes = to_bytes (body) . await . unwrap () ; String :: from_utf8 (bytes . to_vec ()) . unwrap () }
};
}

// Generated macro for get_content_type (function)
macro_rules! Depcrateget_content_type {
() => {
// Module: crate
// Provides: {"get_content_type"}
// Dependencies: {}
fn get_content_type (path : & Path) -> & 'static str { let extension = match path . extension () { None => return "text/plain" , Some (e) => e , } ; match extension . to_str () . unwrap () { "gif" => "image/gif" , "jpg" => "image/jpeg" , "jpeg" => "image/jpeg" , "png" => "image/png" , "pdf" => "application/pdf" , "htm" => "text/html; charset=utf8" , "html" => "text/html; charset=utf8" , "txt" => "text/plain; charset=utf8" , "css" => "text/css; charset=utf8" , _ => "text/plain; charset=utf8" , } }
};
}

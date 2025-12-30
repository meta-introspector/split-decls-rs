// Generated macro for get (function)
macro_rules! Depcrateget {
() => {
// Module: crate
// Provides: {"get"}
// Dependencies: {}
# [doc = " Fetch data from a URL. For more convenient use in _The Rust Programming"] # [doc = " Language_, panics instead of returning a [`Result`] if the request fails."] pub async fn get (url : & str) -> Response { Response (reqwest :: get (url) . await . unwrap ()) }
};
}

// Generated macro for impl_111 (impl)
macro_rules! Depcrate_middleware_redirectimpl_111 {
() => {
// Module: crate::middleware::redirect
// Provides: {"impl_111"}
// Dependencies: {}
impl Redirect { # [doc = " Create a new instance of the Redirect middleware, which attempts to follow redirects"] # [doc = " up to as many times as specified."] # [doc = ""] # [doc = " Consider using `Redirect::default()` for the default number of redirect attempts."] # [doc = ""] # [doc = " This middleware will follow redirects from the `Location` header if the server returns"] # [doc = " any of the following http response codes:"] # [doc = " - 301 Moved Permanently"] # [doc = " - 302 Found"] # [doc = " - 303 See other"] # [doc = " - 307 Temporary Redirect"] # [doc = " - 308 Permanent Redirect"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " An error will be passed through the middleware stack if the value of the `Location`"] # [doc = " header is not a validly parsing url."] # [doc = ""] # [doc = " # Caveats"] # [doc = ""] # [doc = " This will presently make at least one additional HTTP request before the actual request to"] # [doc = " determine if there is a redirect that should be followed, so as to preserve any request body."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[async_std::main]"] # [doc = " # async fn main() -> surf::Result<()> {"] # [doc = " let req = surf::get(\"https://httpbin.org/redirect/2\");"] # [doc = " let client = surf::client().with(surf::middleware::Redirect::new(5));"] # [doc = " let mut res = client.send(req).await?;"] # [doc = " dbg!(res.body_string().await?);"] # [doc = " # Ok(()) }"] # [doc = " ```"] pub fn new (attempts : u8) -> Self { Redirect { attempts } } }
};
}

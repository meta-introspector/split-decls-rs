// Generated macro for echo (function)
macro_rules! Depcrateecho {
() => {
// Module: crate
// Provides: {"echo"}
// Dependencies: {}
async fn echo (req : Request < Incoming >) -> Result < Response < Full < Bytes > > , hyper :: Error > { let response = match (req . method () , req . uri () . path ()) { (& Method :: GET , "/") => { let response = "Hello from HermitOS! 🦀\nTry POST /echo" ; Response :: builder () . header ("Content-Type" , "text/plain; charset=utf8") . header ("content-length" , response . len ()) . body (Full :: from (response . as_bytes ())) . unwrap () } (& Method :: POST , "/echo") => { let data = req . into_body () . collect () . await ? . to_bytes () ; Response :: builder () . header ("Content-Type" , "text/plain; charset=utf8") . header ("content-length" , data . len ()) . body (Full :: from (data)) . unwrap () } _ => Response :: builder () . status (StatusCode :: NOT_FOUND) . body (Full :: default ()) . unwrap () , } ; Ok (response) }
};
}

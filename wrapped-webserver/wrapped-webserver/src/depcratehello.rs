// Generated macro for hello (function)
macro_rules! Depcratehello {
() => {
// Module: crate
// Provides: {"hello"}
// Dependencies: {}
async fn hello (req : Request < Incoming >) -> Result < Response < Full < Bytes > > , hyper :: Error > { let response = match (req . method () , req . uri () . path ()) { (& Method :: GET , path) => { let file_path = if path == "/" { ROOT_DIR . to_string () + "/index.html" } else { ROOT_DIR . to_string () + req . uri () . path () } ; let file_path = Path :: new (& file_path) ; if let Ok (data) = fs :: read (file_path) { Response :: builder () . header ("Content-Type" , get_content_type (file_path)) . header ("content-length" , data . len ()) . body (Full :: from (data)) . unwrap () } else { Response :: builder () . status (StatusCode :: NOT_FOUND) . body (Full :: default ()) . unwrap () } } (& Method :: POST , "/echo") => { let data = req . into_body () . collect () . await ? . to_bytes () ; Response :: builder () . header ("Content-Type" , "text/plain; charset=utf8") . header ("content-length" , data . len ()) . body (Full :: from (data)) . unwrap () } _ => Response :: builder () . status (StatusCode :: NOT_FOUND) . body (Full :: default ()) . unwrap () , } ; Ok (response) }
};
}

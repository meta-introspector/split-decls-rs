// Generated macro for request_compile (function)
macro_rules! Depcrate_commandsrequest_compile {
() => {
// Module: crate::commands
// Provides: {"request_compile"}
// Dependencies: {}
# [doc = " Send a `Compile` request to the server, and return the server response if successful."] fn request_compile < W , X , Y > (conn : & mut ServerConnection , exe : W , args : & [X] , cwd : Y , env_vars : Vec < (OsString , OsString) > ,) -> Result < CompileResponse > where W : AsRef < Path > , X : AsRef < OsStr > , Y : AsRef < Path > , { let req = Request :: Compile (Compile { exe : exe . as_ref () . to_owned () . into () , cwd : cwd . as_ref () . to_owned () . into () , args : args . iter () . map (| a | a . as_ref () . to_owned ()) . collect () , env_vars , }) ; trace ! ("request_compile: {:?}" , req) ; let response = conn . request (req) . context ("Failed to send data to or receive data from server") ? ; if let Response :: Compile (response) = response { Ok (response) } else { bail ! ("Unexpected response from server") } }
};
}

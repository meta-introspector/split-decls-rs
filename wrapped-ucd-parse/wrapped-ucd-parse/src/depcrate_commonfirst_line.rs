// Generated macro for first_line (function)
macro_rules! Depcrate_commonfirst_line {
() => {
// Module: crate::common
// Provides: {"first_line"}
// Dependencies: {}
fn first_line (path : & Path) -> Result < String , Error > { let file = std :: fs :: File :: open (path) . map_err (| e | Error { kind : ErrorKind :: Io (e) , line : None , path : Some (path . into ()) , }) ? ; let mut reader = std :: io :: BufReader :: new (file) ; let mut line_contents = String :: new () ; reader . read_line (& mut line_contents) . map_err (| e | Error { kind : ErrorKind :: Io (e) , line : None , path : Some (path . into ()) , }) ? ; Ok (line_contents) }
};
}

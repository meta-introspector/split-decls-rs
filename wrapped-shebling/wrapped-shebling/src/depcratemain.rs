// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args = Args :: parse () ; let file_path = args . path . to_string_lossy () ; match fs :: read_to_string (& args . path) { Ok (source) => shebling_parser :: parse (& source , & file_path) , Err (err) => { let mut cmd = Args :: command () ; cmd . error (clap :: error :: ErrorKind :: Io , format ! ("{} - {}" , file_path , err) ,) . exit () ; } } }
};
}

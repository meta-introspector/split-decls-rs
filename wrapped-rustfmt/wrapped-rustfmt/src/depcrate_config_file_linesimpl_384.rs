// Generated macro for impl_384 (impl)
macro_rules! Depcrate_config_file_linesimpl_384 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_384"}
// Dependencies: {}
impl fmt :: Display for FileLines { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { None => write ! (f , "None") ? , Some (map) => { for (file_name , ranges) in map . iter () { write ! (f , "{file_name}: ") ? ; write ! (f , "{}\n" , ranges . iter () . format (", ")) ? ; } } } ; Ok (()) } }
};
}

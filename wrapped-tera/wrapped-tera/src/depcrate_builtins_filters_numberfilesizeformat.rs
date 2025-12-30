// Generated macro for filesizeformat (function)
macro_rules! Depcrate_builtins_filters_numberfilesizeformat {
() => {
// Module: crate::builtins::filters::number
// Provides: {"filesizeformat"}
// Dependencies: {}
# [doc = " Returns a human-readable file size (i.e. '110 MB') from an integer"] # [cfg (feature = "builtins")] pub fn filesizeformat (value : & Value , args : & HashMap < String , Value >) -> Result < Value > { let num = try_get_value ! ("filesizeformat" , "value" , usize , value) ; let binary = match args . get ("binary") { Some (binary) => try_get_value ! ("filesizeformat" , "binary" , bool , binary) , None => false , } ; let format = if binary { humansize :: BINARY } else { humansize :: WINDOWS } ; Ok (to_value (format_size (num , format)) . expect ("json serializing should always be possible for a string")) }
};
}

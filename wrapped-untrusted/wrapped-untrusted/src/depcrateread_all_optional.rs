// Generated macro for read_all_optional (function)
macro_rules! Depcrateread_all_optional {
() => {
// Module: crate
// Provides: {"read_all_optional"}
// Dependencies: {}
# [doc = " Calls `read` with the given input as a `Reader`, ensuring that `read`"] # [doc = " consumed the entire input. When `input` is `None`, `read` will be"] # [doc = " called with `None`."] pub fn read_all_optional < 'a , F , R , E > (input : Option < Input < 'a > > , incomplete_read : E , read : F ,) -> Result < R , E > where F : FnOnce (Option < & mut Reader < 'a > >) -> Result < R , E > , { match input { Some (input) => { let mut input = Reader :: new (input) ; let result = read (Some (& mut input)) ? ; if input . at_end () { Ok (result) } else { Err (incomplete_read) } } None => read (None) , } }
};
}

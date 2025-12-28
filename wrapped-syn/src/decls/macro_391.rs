macro_rules! macro_391 {
    () => {
        ast_struct ! { # [doc = " The variadic argument of a foreign function."] # [doc = ""] # [doc = " ```rust"] # [doc = " # struct c_char;"] # [doc = " # struct c_int;"] # [doc = " #"] # [doc = " extern \"C\" {"] # [doc = "     fn printf(format: *const c_char, ...) -> c_int;"] # [doc = "     //                               ^^^"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Variadic { pub attrs : Vec < Attribute >, pub pat : Option < (Box < Pat >, Token ! [:]) >, pub dots : Token ! [...] , pub comma : Option < Token ! [,] >, } }
    };
}

macro_391!();